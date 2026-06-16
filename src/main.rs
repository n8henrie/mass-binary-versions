mod archive;
mod db;
mod error;
mod github;
mod hash;

use std::cmp::Ordering;
use std::fmt::Write;
use std::path::PathBuf;

use clap::{Args, Parser, Subcommand};

use crate::archive::{inspect_downloaded_artifact, inspect_server_source_zip};
use crate::db::{ArtifactMatch, Database, FileMatch, ServerSummaryRow, ServerTagResolution};
use crate::error::{AppError, Result};
use crate::github::{DownloadResult, GitHubClient, Repository, Tag};
use crate::hash::{normalize_sha256, sha256_hex_file};

const DEFAULT_SERVER_REPO: &str = "music-assistant/server";
const README_TABLE_START: &str = "<!-- MA_ARTIFACT_CACHE_TABLE_START -->";
const README_TABLE_END: &str = "<!-- MA_ARTIFACT_CACHE_TABLE_END -->";

#[derive(Debug, Parser)]
#[command(version, about)]
struct Cli {
    #[arg(
        long,
        env = "MA_ARTIFACT_DB",
        default_value = "mass-binary-versions.sqlite3",
        global = true
    )]
    db: PathBuf,

    #[arg(long, env = "GITHUB_TOKEN", hide_env_values = true, global = true)]
    token: Option<String>,

    #[arg(long, env = "GITHUB_API_BASE", global = true)]
    github_api_base: Option<String>,

    #[arg(
        long,
        env = "GITHUB_API_VERSION",
        default_value = "2022-11-28",
        global = true
    )]
    github_api_version: String,

    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Run the full cache update: helper artifacts, server tags, and README table.
    UpdateAll(UpdateAllArgs),

    /// Crawl GitHub Actions artifacts and cache archive/file hashes.
    Crawl(CrawlArgs),

    /// Crawl Music Assistant server tags and hash embedded `AirPlay` binaries.
    CrawlServerTags(CrawlServerTagsArgs),

    /// Join one Music Assistant server tag to cached helper artifact hashes.
    ResolveServerTag(ResolveServerTagArgs),

    /// Print or rewrite the README compatibility table from the database.
    UpdateReadme(UpdateReadmeArgs),

    /// Query the database for an extracted file sha256.
    LookupHash(LookupHashArgs),

    /// Hash local files and query the database for each digest.
    LookupFile(LookupFileArgs),

    /// Query the database for an artifact archive sha256 or GitHub artifact digest.
    LookupArchive(LookupArchiveArgs),

    /// List cached extracted files, optionally filtered by repo, commit, or artifact name.
    ListFiles(ListFilesArgs),

    /// Print database counts.
    Stats,
}

#[allow(clippy::struct_excessive_bools)]
#[derive(Debug, Args)]
struct UpdateAllArgs {
    /// Store metadata only; do not download helper artifact archives or server source zipballs.
    #[arg(long)]
    metadata_only: bool,

    /// Re-download helper artifacts and server source zipballs even if already cached.
    #[arg(long)]
    force: bool,

    /// Stop after this many helper artifact pages per repository. Default: crawl all pages.
    #[arg(long)]
    max_artifact_pages: Option<u32>,

    /// Stop after this many Music Assistant server tag pages. Default: crawl all pages.
    #[arg(long)]
    max_server_tag_pages: Option<u32>,

    /// Results per GitHub API page. GitHub's maximum is 100.
    #[arg(long, default_value_t = 100)]
    per_page: u32,

    /// Return an error on the first artifact or source download/extraction failure.
    #[arg(long)]
    fail_fast: bool,

    /// Music Assistant server repository to crawl.
    #[arg(long, default_value = DEFAULT_SERVER_REPO)]
    server_repo: String,

    /// README path to update after the database crawl succeeds.
    #[arg(long, default_value = "README.md")]
    readme_path: PathBuf,

    /// Maximum number of release rows to include in the README compatibility table.
    #[arg(long, default_value_t = 75)]
    readme_limit: usize,

    /// Do not update README.md after the database crawl succeeds.
    #[arg(long)]
    no_readme: bool,
}

impl Default for UpdateAllArgs {
    fn default() -> Self {
        Self {
            metadata_only: false,
            force: false,
            max_artifact_pages: None,
            max_server_tag_pages: None,
            per_page: 100,
            fail_fast: false,
            server_repo: String::from(DEFAULT_SERVER_REPO),
            readme_path: PathBuf::from("README.md"),
            readme_limit: 75,
            no_readme: false,
        }
    }
}

#[allow(clippy::struct_excessive_bools)]
#[derive(Debug, Args)]
struct CrawlArgs {
    /// Repository to crawl, as OWNER/REPO. Repeatable.
    #[arg(long = "repo")]
    repos: Vec<String>,

    /// Artifact name prefix to include. Repeatable. Defaults to cliraop- and cliap2-.
    #[arg(long = "artifact-prefix")]
    artifact_prefixes: Vec<String>,

    /// Include all artifacts instead of applying name prefixes.
    #[arg(long)]
    all_artifacts: bool,

    /// List and store artifact metadata but do not download archives.
    #[arg(long)]
    metadata_only: bool,

    /// Re-download artifacts even if `archive_sha256` is already cached.
    #[arg(long)]
    force: bool,

    /// Stop after this many pages per repository. Default: crawl all pages.
    #[arg(long)]
    max_pages: Option<u32>,

    /// Results per GitHub API page. GitHub's maximum is 100.
    #[arg(long, default_value_t = 100)]
    per_page: u32,

    /// Return an error on the first artifact download or extraction failure.
    #[arg(long)]
    fail_fast: bool,
}

#[derive(Debug, Args)]
struct CrawlServerTagsArgs {
    /// Music Assistant server repository to crawl.
    #[arg(long, default_value = DEFAULT_SERVER_REPO)]
    repo: String,

    /// Specific tag to crawl. Repeatable. Default: crawl all tags.
    #[arg(long = "tag")]
    tags: Vec<String>,

    /// Store tag metadata but do not download source zipballs.
    #[arg(long)]
    metadata_only: bool,

    /// Re-download source zipballs even if already cached.
    #[arg(long)]
    force: bool,

    /// Stop after this many tag pages. Default: crawl all pages.
    #[arg(long)]
    max_pages: Option<u32>,

    /// Tags per GitHub API page. GitHub's maximum is 100.
    #[arg(long, default_value_t = 100)]
    per_page: u32,

    /// Return an error on the first source download or extraction failure.
    #[arg(long)]
    fail_fast: bool,
}

#[derive(Debug, Args)]
struct ResolveServerTagArgs {
    /// Music Assistant server tag, for example 2.8.7.
    tag: String,

    /// Music Assistant server repository.
    #[arg(long, default_value = DEFAULT_SERVER_REPO)]
    repo: String,
}

#[derive(Debug, Args)]
struct UpdateReadmeArgs {
    /// README path to update.
    #[arg(long, default_value = "README.md")]
    path: PathBuf,

    /// Music Assistant server repository.
    #[arg(long, default_value = DEFAULT_SERVER_REPO)]
    repo: String,

    /// Maximum number of release rows to include.
    #[arg(long, default_value_t = 50)]
    limit: usize,

    /// Print the generated table instead of editing the README.
    #[arg(long)]
    stdout: bool,
}

#[derive(Debug, Args)]
struct LookupHashArgs {
    /// Raw sha256 digest, optionally prefixed by sha256:.
    sha256: String,
}

#[derive(Debug, Args)]
struct LookupFileArgs {
    /// File to hash and look up. Repeatable.
    paths: Vec<PathBuf>,
}

#[derive(Debug, Args)]
struct LookupArchiveArgs {
    /// Archive sha256 or GitHub artifact digest, optionally prefixed by sha256:.
    sha256: String,
}

#[derive(Debug, Args)]
struct ListFilesArgs {
    #[arg(long)]
    repo: Option<String>,

    #[arg(long)]
    head_sha: Option<String>,

    #[arg(long)]
    artifact_name: Option<String>,

    #[arg(long, default_value_t = 100)]
    limit: i64,
}

#[derive(Default)]
struct CrawlTotals {
    seen: u64,
    selected: u64,
    downloaded: u64,
    metadata_only: u64,
    skipped_downloaded: u64,
    skipped_name: u64,
    expired: u64,
    errors: u64,
}

#[derive(Default)]
struct ServerTagTotals {
    seen: u64,
    downloaded: u64,
    metadata_only: u64,
    skipped_downloaded: u64,
    errors: u64,
    binaries: u64,
}

#[derive(Debug, Clone, Copy)]
enum ArtifactDownloadOutcome {
    Downloaded,
    Gone,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    let database = Database::open(&cli.db).await?;

    let command = match cli.command {
        Some(command) => command,
        None => Command::UpdateAll(UpdateAllArgs::default()),
    };

    match command {
        Command::UpdateAll(args) => {
            let github = GitHubClient::new(
                cli.token.as_deref(),
                cli.github_api_base.as_deref(),
                Some(&cli.github_api_version),
            )?;
            update_all(&database, &github, &args).await?;
        }
        Command::Crawl(args) => {
            let github = GitHubClient::new(
                cli.token.as_deref(),
                cli.github_api_base.as_deref(),
                Some(&cli.github_api_version),
            )?;
            crawl(&database, &github, &args).await?;
        }
        Command::CrawlServerTags(args) => {
            let github = GitHubClient::new(
                cli.token.as_deref(),
                cli.github_api_base.as_deref(),
                Some(&cli.github_api_version),
            )?;
            crawl_server_tags(&database, &github, &args).await?;
        }
        Command::ResolveServerTag(args) => {
            let rows = database.resolve_server_tag(&args.repo, &args.tag).await?;
            print_server_tag_resolution(&rows);
        }
        Command::UpdateReadme(args) => {
            update_readme(&database, &args).await?;
        }
        Command::LookupHash(args) => {
            let digest = normalize_sha256(&args.sha256)?;
            let matches = database.query_file_hash(&digest).await?;
            print_file_matches(&matches);
        }
        Command::LookupFile(args) => {
            lookup_files(&database, &args.paths).await?;
        }
        Command::LookupArchive(args) => {
            let digest = normalize_sha256(&args.sha256)?;
            let matches = database.query_archive_hash(&digest).await?;
            print_artifact_matches(&matches);
        }
        Command::ListFiles(args) => {
            let matches = database
                .list_files(
                    args.repo.as_deref(),
                    args.head_sha.as_deref(),
                    args.artifact_name.as_deref(),
                    args.limit,
                )
                .await?;
            print_file_matches(&matches);
        }
        Command::Stats => {
            let stats = database.stats().await?;
            println!("artifacts\t{}", stats.artifacts);
            println!("downloaded_artifacts\t{}", stats.downloaded_artifacts);
            println!("expired_artifacts\t{}", stats.expired_artifacts);
            println!("files\t{}", stats.files);
            println!("repos\t{}", stats.repos);
            println!("commits\t{}", stats.commits);
            println!("server_tags\t{}", stats.server_tags);
            println!("server_tag_binaries\t{}", stats.server_tag_binaries);
        }
    }

    Ok(())
}

async fn update_all(
    database: &Database,
    github: &GitHubClient,
    args: &UpdateAllArgs,
) -> Result<()> {
    let per_page = args.per_page.clamp(1, 100);
    let libraop_args = CrawlArgs {
        repos: vec![String::from("music-assistant/libraop")],
        artifact_prefixes: vec![String::from("cliraop-")],
        all_artifacts: false,
        metadata_only: args.metadata_only,
        force: args.force,
        max_pages: args.max_artifact_pages,
        per_page,
        fail_fast: args.fail_fast,
    };
    let cliairplay_args = CrawlArgs {
        repos: vec![String::from("music-assistant/cliairplay")],
        artifact_prefixes: vec![String::from("cliap2-")],
        all_artifacts: false,
        metadata_only: args.metadata_only,
        force: args.force,
        max_pages: args.max_artifact_pages,
        per_page,
        fail_fast: args.fail_fast,
    };
    let server_args = CrawlServerTagsArgs {
        repo: args.server_repo.clone(),
        tags: Vec::new(),
        metadata_only: args.metadata_only,
        force: args.force,
        max_pages: args.max_server_tag_pages,
        per_page,
        fail_fast: args.fail_fast,
    };

    eprintln!("updating helper artifacts and Music Assistant server tags concurrently");
    tokio::try_join!(
        crawl(database, github, &libraop_args),
        crawl(database, github, &cliairplay_args),
        crawl_server_tags(database, github, &server_args),
    )?;

    if !args.no_readme {
        let readme_args = UpdateReadmeArgs {
            path: args.readme_path.clone(),
            repo: args.server_repo.clone(),
            limit: args.readme_limit,
            stdout: false,
        };
        update_readme(database, &readme_args).await?;
    }

    Ok(())
}

async fn crawl(database: &Database, github: &GitHubClient, args: &CrawlArgs) -> Result<()> {
    let repositories = repository_args(args)?;
    let prefixes = artifact_prefixes(args);
    let per_page = args.per_page.clamp(1, 100);
    let mut totals = CrawlTotals::default();

    for repository in repositories {
        eprintln!("crawling {}", repository.full_name);
        crawl_repository(
            database,
            github,
            args,
            &repository,
            &prefixes,
            per_page,
            &mut totals,
        )
        .await?;
    }

    eprintln!(
        "done: seen={} selected={} downloaded={} metadata_only={} skipped_downloaded={} skipped_name={} expired={} errors={}",
        totals.seen,
        totals.selected,
        totals.downloaded,
        totals.metadata_only,
        totals.skipped_downloaded,
        totals.skipped_name,
        totals.expired,
        totals.errors
    );

    Ok(())
}

async fn crawl_repository(
    database: &Database,
    github: &GitHubClient,
    args: &CrawlArgs,
    repository: &Repository,
    prefixes: &[String],
    per_page: u32,
    totals: &mut CrawlTotals,
) -> Result<()> {
    let mut page = 1_u32;

    loop {
        if should_stop_at_page(args.max_pages, page) {
            break;
        }

        let artifact_page = github.list_artifacts(repository, page, per_page).await?;
        if artifact_page.artifacts.is_empty() {
            break;
        }

        for artifact in artifact_page.artifacts {
            totals.seen += 1;
            database
                .upsert_artifact(&repository.full_name, &artifact)
                .await?;

            if !artifact_selected(&artifact.name, args.all_artifacts, prefixes) {
                totals.skipped_name += 1;
                continue;
            }
            totals.selected += 1;

            if artifact.expired {
                totals.expired += 1;
                continue;
            }

            if args.metadata_only {
                totals.metadata_only += 1;
                continue;
            }

            if !args.force
                && database
                    .artifact_has_download(&repository.full_name, artifact.id)
                    .await?
            {
                totals.skipped_downloaded += 1;
                continue;
            }

            match process_artifact_download(database, github, repository, &artifact).await {
                Ok(ArtifactDownloadOutcome::Downloaded) => totals.downloaded += 1,
                Ok(ArtifactDownloadOutcome::Gone) => totals.expired += 1,
                Err(error) => {
                    totals.errors += 1;
                    database
                        .mark_download_error(&repository.full_name, artifact.id, &error.to_string())
                        .await?;
                    if args.fail_fast {
                        return Err(error);
                    }
                    eprintln!(
                        "error: {} artifact {}#{}: {error}",
                        repository.full_name, artifact.name, artifact.id
                    );
                }
            }
        }

        if reached_last_page(page, per_page, artifact_page.total_count) {
            break;
        }
        page += 1;
    }

    Ok(())
}

async fn process_artifact_download(
    database: &Database,
    github: &GitHubClient,
    repository: &Repository,
    artifact: &github::Artifact,
) -> Result<ArtifactDownloadOutcome> {
    match github.download_artifact(artifact).await? {
        DownloadResult::Gone => {
            database
                .mark_gone(&repository.full_name, artifact.id)
                .await?;
            Ok(ArtifactDownloadOutcome::Gone)
        }
        DownloadResult::Bytes(bytes) => {
            let inspected = inspect_downloaded_artifact(
                &repository.full_name,
                artifact.id,
                &artifact.name,
                &bytes,
            )?;
            database
                .mark_downloaded(&repository.full_name, artifact.id, &inspected)
                .await?;
            eprintln!(
                "downloaded {}#{} {} files={} archive_sha256={}",
                repository.full_name,
                artifact.id,
                artifact.name,
                inspected.files.len(),
                inspected.archive_sha256
            );
            Ok(ArtifactDownloadOutcome::Downloaded)
        }
    }
}

async fn crawl_server_tags(
    database: &Database,
    github: &GitHubClient,
    args: &CrawlServerTagsArgs,
) -> Result<()> {
    let repository = Repository::parse(&args.repo)?;
    let per_page = args.per_page.clamp(1, 100);
    let tags = server_tags(github, &repository, args, per_page).await?;
    let mut totals = ServerTagTotals::default();

    eprintln!("crawling {} server tags", repository.full_name);
    for tag in tags {
        totals.seen += 1;
        database
            .upsert_server_tag_metadata(&repository.full_name, &tag)
            .await?;

        if args.metadata_only {
            totals.metadata_only += 1;
            continue;
        }

        if !args.force
            && database
                .server_tag_has_download(&repository.full_name, &tag.name)
                .await?
        {
            totals.skipped_downloaded += 1;
            continue;
        }

        match process_server_tag_download(database, github, &repository, &tag).await {
            Ok(binary_count) => {
                totals.downloaded += 1;
                totals.binaries += binary_count;
            }
            Err(error) => {
                totals.errors += 1;
                database
                    .mark_server_tag_error(&repository.full_name, &tag.name, &error.to_string())
                    .await?;
                if args.fail_fast {
                    return Err(error);
                }
                eprintln!("error: {} tag {}: {error}", repository.full_name, tag.name);
            }
        }
    }

    eprintln!(
        "done: seen={} downloaded={} binaries={} metadata_only={} skipped_downloaded={} errors={}",
        totals.seen,
        totals.downloaded,
        totals.binaries,
        totals.metadata_only,
        totals.skipped_downloaded,
        totals.errors
    );

    Ok(())
}

async fn server_tags(
    github: &GitHubClient,
    repository: &Repository,
    args: &CrawlServerTagsArgs,
    per_page: u32,
) -> Result<Vec<Tag>> {
    if !args.tags.is_empty() {
        let mut tags = Vec::with_capacity(args.tags.len());
        for tag_name in &args.tags {
            tags.push(github.resolve_tag(repository, tag_name).await?);
        }
        return Ok(tags);
    }

    let mut tags = Vec::new();
    let mut page = 1_u32;

    loop {
        if should_stop_at_page(args.max_pages, page) {
            break;
        }

        let page_tags = github.list_tags(repository, page, per_page).await?;
        if page_tags.is_empty() {
            break;
        }

        let page_len = page_tags.len();
        tags.extend(page_tags);
        if page_len < usize::try_from(per_page).map_or(usize::MAX, |value| value) {
            break;
        }

        page += 1;
    }

    Ok(tags)
}

async fn process_server_tag_download(
    database: &Database,
    github: &GitHubClient,
    repository: &Repository,
    tag: &Tag,
) -> Result<u64> {
    match github.download_tag_zipball(tag).await? {
        DownloadResult::Gone => Err(AppError::GitHubStatus {
            status: reqwest::StatusCode::GONE,
            url: tag.zipball_url.clone(),
            body: String::from("tag zipball gone"),
        }),
        DownloadResult::Bytes(bytes) => {
            let inspected = inspect_server_source_zip(&repository.full_name, &tag.name, &bytes)?;
            let binary_count = u64::try_from(inspected.binaries.len()).map_or(0, |value| value);
            database
                .mark_server_tag_downloaded(&repository.full_name, tag, &inspected)
                .await?;
            eprintln!(
                "downloaded {}@{} rev={} binaries={} source_zip_sha256={}",
                repository.full_name,
                tag.name,
                tag.commit.sha,
                inspected.binaries.len(),
                inspected.source_zip_sha256
            );
            Ok(binary_count)
        }
    }
}

async fn update_readme(database: &Database, args: &UpdateReadmeArgs) -> Result<()> {
    let table = compatibility_table(database, &args.repo, args.limit).await?;
    if args.stdout {
        print!("{table}");
        return Ok(());
    }

    let existing = match tokio::fs::read_to_string(&args.path).await {
        Ok(value) => value,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => String::new(),
        Err(error) => return Err(error.into()),
    };
    let updated = replace_or_append_readme_table(&existing, &table)?;
    tokio::fs::write(&args.path, updated).await?;
    eprintln!("updated {}", args.path.display());
    Ok(())
}

async fn compatibility_table(database: &Database, repo: &str, limit: usize) -> Result<String> {
    let mut rows = database.server_summary_rows(repo).await?;
    rows.sort_by(|left, right| compare_tag_desc(&left.tag_name, &right.tag_name));
    if rows.len() > limit {
        rows.truncate(limit);
    }

    let mut output = String::from(
        "| Music Assistant release | server rev | cliairplay commit | libraop commit |\n|---|---:|---:|---:|\n",
    );

    if rows.is_empty() {
        output.push_str("| _No Music Assistant server tags cached yet._ |  |  |  |\n");
        return Ok(output);
    }

    for row in rows {
        writeln!(
            output,
            "| `{}` | {} | {} | {} |",
            row.tag_name,
            markdown_commit(&row.tag_repo, &row.server_commit_sha),
            helper_commit_cell(&row, "cliairplay")?,
            helper_commit_cell(&row, "libraop")?
        )?;
    }

    Ok(output)
}

fn replace_or_append_readme_table(existing: &str, table: &str) -> Result<String> {
    let block = format!(
        "{README_TABLE_START}\n{}\n{README_TABLE_END}",
        table.trim_end()
    );

    if let Some(start) = existing.find(README_TABLE_START) {
        let search_from_start = &existing[start..];
        let Some(relative_end) = search_from_start.find(README_TABLE_END) else {
            return Err(AppError::MissingReadmeMarker(README_TABLE_END.to_owned()));
        };
        let end = start + relative_end + README_TABLE_END.len();
        let mut updated = String::new();
        updated.push_str(&existing[..start]);
        updated.push_str(&block);
        updated.push_str(&existing[end..]);
        Ok(updated)
    } else {
        let mut updated = existing.trim_end().to_owned();
        if !updated.is_empty() {
            updated.push_str("\n\n");
        }
        updated.push_str("## Music Assistant AirPlay compatibility\n\n");
        updated.push_str(&block);
        updated.push('\n');
        Ok(updated)
    }
}

fn helper_commit_cell(row: &ServerSummaryRow, helper_package: &str) -> Result<String> {
    let (commits, binary_count, match_count, repo) = if helper_package == "cliairplay" {
        (
            &row.cliairplay_commits,
            row.cliap2_binary_count,
            row.cliap2_match_count,
            "music-assistant/cliairplay",
        )
    } else {
        (
            &row.libraop_commits,
            row.cliraop_binary_count,
            row.cliraop_match_count,
            "music-assistant/libraop",
        )
    };

    if commits.is_empty() {
        return if binary_count > 0 {
            Ok(format!("missing ({match_count}/{binary_count} hashes)"))
        } else {
            Ok(String::from("none"))
        };
    }

    let mut formatted = commits
        .iter()
        .map(|commit| markdown_commit(repo, commit))
        .collect::<Vec<_>>()
        .join("<br>");

    if match_count < binary_count {
        write!(
            formatted,
            "<br>partial ({match_count}/{binary_count} hashes)"
        )?;
    }

    Ok(formatted)
}

fn markdown_commit(repo: &str, commit: &str) -> String {
    format!(
        "[`{}`](https://github.com/{repo}/commit/{commit})",
        short_sha(commit)
    )
}

fn short_sha(sha: &str) -> &str {
    match sha.get(..12) {
        Some(value) => value,
        None => sha,
    }
}

fn compare_tag_desc(left: &str, right: &str) -> Ordering {
    compare_version_like(right, left).then_with(|| right.cmp(left))
}

fn compare_version_like(left: &str, right: &str) -> Ordering {
    let left_parts = numeric_parts(left);
    let right_parts = numeric_parts(right);
    let max_len = left_parts.len().max(right_parts.len());

    for index in 0..max_len {
        let left_part = match left_parts.get(index) {
            Some(value) => *value,
            None => 0,
        };
        let right_part = match right_parts.get(index) {
            Some(value) => *value,
            None => 0,
        };
        let ordering = left_part.cmp(&right_part);
        if ordering != Ordering::Equal {
            return ordering;
        }
    }

    Ordering::Equal
}

fn numeric_parts(value: &str) -> Vec<i64> {
    let mut parts = Vec::new();
    let mut current = String::new();

    for character in value.chars() {
        if character.is_ascii_digit() {
            current.push(character);
        } else if !current.is_empty() {
            if let Ok(number) = current.parse::<i64>() {
                parts.push(number);
            }
            current.clear();
        }
    }

    if !current.is_empty()
        && let Ok(number) = current.parse::<i64>()
    {
        parts.push(number);
    }

    parts
}

async fn lookup_files(database: &Database, paths: &[PathBuf]) -> Result<()> {
    for path in paths {
        let digest = sha256_hex_file(path).await?;
        println!("# {}\t{}", path.display(), digest);
        let matches = database.query_file_hash(&digest).await?;
        print_file_matches(&matches);
    }
    Ok(())
}

fn repository_args(args: &CrawlArgs) -> Result<Vec<Repository>> {
    let values = if args.repos.is_empty() {
        default_repositories()
    } else {
        args.repos.clone()
    };

    values
        .iter()
        .map(|value| Repository::parse(value))
        .collect()
}

fn default_repositories() -> Vec<String> {
    vec![
        String::from("music-assistant/libraop"),
        String::from("music-assistant/cliairplay"),
    ]
}

fn artifact_prefixes(args: &CrawlArgs) -> Vec<String> {
    if args.all_artifacts {
        Vec::new()
    } else if args.artifact_prefixes.is_empty() {
        vec![String::from("cliraop-"), String::from("cliap2-")]
    } else {
        args.artifact_prefixes.clone()
    }
}

fn artifact_selected(name: &str, all_artifacts: bool, prefixes: &[String]) -> bool {
    all_artifacts || prefixes.iter().any(|prefix| name.starts_with(prefix))
}

fn should_stop_at_page(max_pages: Option<u32>, page: u32) -> bool {
    match max_pages {
        Some(max_pages) => page > max_pages,
        None => false,
    }
}

fn reached_last_page(page: u32, per_page: u32, total_count: u64) -> bool {
    u64::from(page) * u64::from(per_page) >= total_count
}

fn print_file_matches(matches: &[FileMatch]) {
    if matches.is_empty() {
        println!("no matches");
        return;
    }

    println!(
        "repo\tartifact_id\tartifact_name\tfile_name\tfile_sha256\tfile_size_bytes\thead_sha\thead_branch\trun_id\tcreated_at\texpires_at\texpired\tarchive_sha256\tarchive_size_bytes\tartifact_digest"
    );
    for entry in matches {
        println!(
            "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}",
            entry.repo,
            entry.artifact_id,
            entry.artifact_name,
            entry.file_name,
            entry.file_sha256,
            entry.file_size_bytes,
            display_option(entry.head_sha.as_deref()),
            display_option(entry.head_branch.as_deref()),
            display_option_i64(entry.run_id),
            display_option(entry.created_at.as_deref()),
            display_option(entry.expires_at.as_deref()),
            entry.expired,
            display_option(entry.archive_sha256.as_deref()),
            display_option_i64(entry.archive_size_bytes),
            display_option(entry.artifact_digest.as_deref())
        );
    }
}

fn print_artifact_matches(matches: &[ArtifactMatch]) {
    if matches.is_empty() {
        println!("no matches");
        return;
    }

    println!(
        "repo\tartifact_id\tartifact_name\thead_sha\thead_branch\trun_id\tcreated_at\texpires_at\texpired\tarchive_sha256\tarchive_size_bytes\tartifact_digest"
    );
    for entry in matches {
        println!(
            "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}",
            entry.repo,
            entry.artifact_id,
            entry.artifact_name,
            display_option(entry.head_sha.as_deref()),
            display_option(entry.head_branch.as_deref()),
            display_option_i64(entry.run_id),
            display_option(entry.created_at.as_deref()),
            display_option(entry.expires_at.as_deref()),
            entry.expired,
            display_option(entry.archive_sha256.as_deref()),
            display_option_i64(entry.archive_size_bytes),
            display_option(entry.artifact_digest.as_deref())
        );
    }
}

fn print_server_tag_resolution(rows: &[ServerTagResolution]) {
    if rows.is_empty() {
        println!("no matches");
        return;
    }

    println!(
        "tag_repo\ttag_name\tserver_commit_sha\tbinary_name\thelper_package\thelper_repo\tbinary_sha256\thelper_commit\tartifact_name\tartifact_id\trun_id\tartifact_created_at"
    );
    for row in rows {
        println!(
            "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}",
            row.tag_repo,
            row.tag_name,
            row.server_commit_sha,
            row.binary_name,
            row.helper_package,
            row.helper_repo,
            row.binary_sha256,
            display_option(row.helper_commit.as_deref()),
            display_option(row.artifact_name.as_deref()),
            display_option_i64(row.artifact_id),
            display_option_i64(row.run_id),
            display_option(row.artifact_created_at.as_deref())
        );
    }
}

fn display_option(value: Option<&str>) -> &str {
    value.unwrap_or_default()
}

fn display_option_i64(value: Option<i64>) -> String {
    match value {
        Some(value) => value.to_string(),
        None => String::new(),
    }
}

#[cfg(test)]
mod tests {
    use std::cmp::Ordering;

    use super::{
        README_TABLE_END, README_TABLE_START, compare_version_like, replace_or_append_readme_table,
    };

    #[test]
    fn compares_version_like_tags() {
        assert_eq!(compare_version_like("2.8.10", "2.8.7"), Ordering::Greater);
        assert_eq!(compare_version_like("2.8.7", "2.8.7"), Ordering::Equal);
    }

    #[test]
    fn replaces_readme_marked_table() {
        let original = format!("a\n{README_TABLE_START}\nold\n{README_TABLE_END}\nz\n");
        match replace_or_append_readme_table(&original, "new\n") {
            Ok(updated) => assert_eq!(
                updated,
                format!("a\n{README_TABLE_START}\nnew\n{README_TABLE_END}\nz\n")
            ),
            Err(error) => panic!("replacement should succeed: {error}"),
        }
    }
}
