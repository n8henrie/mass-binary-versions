use reqwest::StatusCode;
use reqwest::header::InvalidHeaderValue;
use thiserror::Error;

pub(crate) type Result<T> = std::result::Result<T, AppError>;

#[derive(Debug, Error)]
pub(crate) enum AppError {
    #[error("repository must be OWNER/REPO, got {0:?}")]
    InvalidRepository(String),

    #[error(
        "invalid sha256 digest {0:?}; expected 64 hex characters, optionally prefixed by sha256:"
    )]
    InvalidSha256(String),

    #[error("GitHub API request failed: {status} {url}: {body}")]
    GitHubStatus {
        status: StatusCode,
        url: String,
        body: String,
    },

    #[error("artifact {repo}#{artifact_id} downloaded as a ZIP, but it contained no regular files")]
    EmptyArtifact { repo: String, artifact_id: i64 },

    #[error("artifact {repo}#{artifact_id} was too large to store length as i64")]
    ArtifactTooLarge { repo: String, artifact_id: i64 },

    #[error(
        "Music Assistant source zipball {repo}@{tag_name} was too large to store length as i64"
    )]
    SourceZipTooLarge { repo: String, tag_name: String },

    #[error("Music Assistant source download {repo}@{tag_name} did not look like a ZIP archive")]
    NotZipball { repo: String, tag_name: String },

    #[error("README marker {0:?} was found without matching end marker")]
    MissingReadmeMarker(String),

    #[error(transparent)]
    HeaderValue(#[from] InvalidHeaderValue),

    #[error(transparent)]
    Http(#[from] reqwest::Error),

    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    Fmt(#[from] std::fmt::Error),

    #[error(transparent)]
    Json(#[from] serde_json::Error),

    #[error(transparent)]
    Sql(#[from] sqlx::Error),

    #[error(transparent)]
    Zip(#[from] zip::result::ZipError),
}
