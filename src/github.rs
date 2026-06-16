use reqwest::header::{ACCEPT, AUTHORIZATION, HeaderMap, HeaderName, HeaderValue, USER_AGENT};
use reqwest::{Client, StatusCode};
use serde::{Deserialize, Serialize};

use crate::error::{AppError, Result};

const DEFAULT_API_BASE: &str = "https://api.github.com";
const DEFAULT_API_VERSION: &str = "2022-11-28";

#[derive(Clone, Debug)]
pub(crate) struct Repository {
    pub(crate) full_name: String,
    owner: String,
    name: String,
}

impl Repository {
    pub(crate) fn parse(value: &str) -> Result<Self> {
        let mut parts = value.split('/');
        let Some(owner) = parts.next() else {
            return Err(AppError::InvalidRepository(value.to_owned()));
        };
        let Some(name) = parts.next() else {
            return Err(AppError::InvalidRepository(value.to_owned()));
        };
        if parts.next().is_some() || owner.is_empty() || name.is_empty() {
            return Err(AppError::InvalidRepository(value.to_owned()));
        }

        Ok(Self {
            full_name: format!("{owner}/{name}"),
            owner: owner.to_owned(),
            name: name.to_owned(),
        })
    }
}

#[derive(Clone, Debug)]
pub(crate) struct GitHubClient {
    client: Client,
    api_base: String,
}

impl GitHubClient {
    pub(crate) fn new(
        token: Option<&str>,
        api_base: Option<&str>,
        api_version: Option<&str>,
    ) -> Result<Self> {
        let mut headers = HeaderMap::new();
        headers.insert(USER_AGENT, HeaderValue::from_static("mass-binary-versions"));
        headers.insert(
            ACCEPT,
            HeaderValue::from_static("application/vnd.github+json"),
        );
        headers.insert(
            HeaderName::from_static("x-github-api-version"),
            HeaderValue::from_str(match api_version {
                Some(value) => value,
                None => DEFAULT_API_VERSION,
            })?,
        );

        if let Some(token) = token.filter(|value| !value.is_empty()) {
            let value = format!("Bearer {token}");
            headers.insert(AUTHORIZATION, HeaderValue::from_str(&value)?);
        }

        let client = Client::builder()
            .default_headers(headers)
            .redirect(reqwest::redirect::Policy::limited(10))
            .build()?;

        Ok(Self {
            client,
            api_base: match api_base {
                Some(value) => value,
                None => DEFAULT_API_BASE,
            }
            .trim_end_matches('/')
            .to_owned(),
        })
    }

    pub(crate) async fn list_artifacts(
        &self,
        repository: &Repository,
        page: u32,
        per_page: u32,
    ) -> Result<ArtifactsPage> {
        let url = format!(
            "{}/repos/{}/{}/actions/artifacts?per_page={per_page}&page={page}",
            self.api_base, repository.owner, repository.name
        );
        let response = self.client.get(&url).send().await?;
        response_json(response, &url).await
    }

    pub(crate) async fn download_artifact(&self, artifact: &Artifact) -> Result<DownloadResult> {
        self.download_url(&artifact.archive_download_url).await
    }

    pub(crate) async fn list_tags(
        &self,
        repository: &Repository,
        page: u32,
        per_page: u32,
    ) -> Result<Vec<Tag>> {
        let url = format!(
            "{}/repos/{}/{}/tags?per_page={per_page}&page={page}",
            self.api_base, repository.owner, repository.name
        );
        let response = self.client.get(&url).send().await?;
        response_json(response, &url).await
    }

    pub(crate) async fn resolve_tag(&self, repository: &Repository, tag_name: &str) -> Result<Tag> {
        let url = format!(
            "{}/repos/{}/{}/commits/{}",
            self.api_base, repository.owner, repository.name, tag_name
        );
        let response = self.client.get(&url).send().await?;
        let commit: CommitLookup = response_json(response, &url).await?;

        Ok(Tag {
            name: tag_name.to_owned(),
            commit: TagCommit {
                sha: commit.sha,
                url: Some(url),
            },
            zipball_url: format!(
                "{}/repos/{}/{}/zipball/{}",
                self.api_base, repository.owner, repository.name, tag_name
            ),
            tarball_url: format!(
                "{}/repos/{}/{}/tarball/{}",
                self.api_base, repository.owner, repository.name, tag_name
            ),
            node_id: None,
        })
    }

    pub(crate) async fn download_tag_zipball(&self, tag: &Tag) -> Result<DownloadResult> {
        self.download_url(&tag.zipball_url).await
    }

    async fn download_url(&self, url: &str) -> Result<DownloadResult> {
        let response = self.client.get(url).send().await?;
        let status = response.status();

        if status == StatusCode::GONE {
            return Ok(DownloadResult::Gone);
        }

        if !status.is_success() {
            let body = match response.text().await {
                Ok(value) => value,
                Err(error) => format!("<failed to read body: {error}>"),
            };
            return Err(AppError::GitHubStatus {
                status,
                url: url.to_owned(),
                body,
            });
        }

        let bytes = response.bytes().await?.to_vec();
        Ok(DownloadResult::Bytes(bytes))
    }
}

pub(crate) enum DownloadResult {
    Bytes(Vec<u8>),
    Gone,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub(crate) struct ArtifactsPage {
    pub(crate) total_count: u64,
    pub(crate) artifacts: Vec<Artifact>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub(crate) struct Artifact {
    pub(crate) id: i64,
    pub(crate) node_id: Option<String>,
    pub(crate) name: String,
    pub(crate) size_in_bytes: Option<i64>,
    pub(crate) url: String,
    pub(crate) archive_download_url: String,
    pub(crate) expired: bool,
    pub(crate) created_at: Option<String>,
    pub(crate) expires_at: Option<String>,
    pub(crate) updated_at: Option<String>,
    pub(crate) digest: Option<String>,
    pub(crate) workflow_run: Option<WorkflowRun>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub(crate) struct WorkflowRun {
    pub(crate) id: Option<i64>,
    pub(crate) repository_id: Option<i64>,
    pub(crate) head_repository_id: Option<i64>,
    pub(crate) head_branch: Option<String>,
    pub(crate) head_sha: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub(crate) struct Tag {
    pub(crate) name: String,
    pub(crate) commit: TagCommit,
    pub(crate) zipball_url: String,
    pub(crate) tarball_url: String,
    pub(crate) node_id: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub(crate) struct TagCommit {
    pub(crate) sha: String,
    pub(crate) url: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
struct CommitLookup {
    sha: String,
}

async fn response_json<T>(response: reqwest::Response, url: &str) -> Result<T>
where
    T: serde::de::DeserializeOwned,
{
    let status = response.status();
    if !status.is_success() {
        let body = match response.text().await {
            Ok(value) => value,
            Err(error) => format!("<failed to read body: {error}>"),
        };
        return Err(AppError::GitHubStatus {
            status,
            url: url.to_owned(),
            body,
        });
    }

    Ok(response.json::<T>().await?)
}

#[cfg(test)]
mod tests {
    use super::Repository;

    #[test]
    fn parses_repo() {
        match Repository::parse("music-assistant/libraop") {
            Ok(repo) => assert_eq!(repo.full_name, "music-assistant/libraop"),
            Err(error) => panic!("valid repository should parse: {error}"),
        }
    }

    #[test]
    fn rejects_bad_repo() {
        assert!(Repository::parse("libraop").is_err());
        assert!(Repository::parse("a/b/c").is_err());
    }
}
