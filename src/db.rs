use std::path::Path;
use std::str::FromStr;
use std::time::Duration;

use sqlx::sqlite::{SqliteConnectOptions, SqliteJournalMode, SqlitePoolOptions};
use sqlx::{QueryBuilder, Row, Sqlite, SqlitePool};

use crate::archive::{ArtifactArchive, ArtifactFile, ServerBinary, ServerSourceArchive};
use crate::error::Result;
use crate::github::{Artifact, Tag};

#[derive(Clone)]
pub(crate) struct Database {
    pool: SqlitePool,
}

#[derive(Debug, Clone)]
pub(crate) struct FileMatch {
    pub(crate) repo: String,
    pub(crate) artifact_id: i64,
    pub(crate) artifact_name: String,
    pub(crate) artifact_digest: Option<String>,
    pub(crate) archive_sha256: Option<String>,
    pub(crate) archive_size_bytes: Option<i64>,
    pub(crate) file_name: String,
    pub(crate) file_sha256: String,
    pub(crate) file_size_bytes: i64,
    pub(crate) run_id: Option<i64>,
    pub(crate) head_sha: Option<String>,
    pub(crate) head_branch: Option<String>,
    pub(crate) created_at: Option<String>,
    pub(crate) expires_at: Option<String>,
    pub(crate) expired: bool,
}

#[derive(Debug, Clone)]
pub(crate) struct ArtifactMatch {
    pub(crate) repo: String,
    pub(crate) artifact_id: i64,
    pub(crate) artifact_name: String,
    pub(crate) artifact_digest: Option<String>,
    pub(crate) archive_sha256: Option<String>,
    pub(crate) archive_size_bytes: Option<i64>,
    pub(crate) run_id: Option<i64>,
    pub(crate) head_sha: Option<String>,
    pub(crate) head_branch: Option<String>,
    pub(crate) created_at: Option<String>,
    pub(crate) expires_at: Option<String>,
    pub(crate) expired: bool,
}

#[derive(Debug, Clone)]
pub(crate) struct ServerTagResolution {
    pub(crate) tag_repo: String,
    pub(crate) tag_name: String,
    pub(crate) server_commit_sha: String,
    pub(crate) binary_name: String,
    pub(crate) helper_package: String,
    pub(crate) helper_repo: String,
    pub(crate) binary_sha256: String,
    pub(crate) helper_commit: Option<String>,
    pub(crate) artifact_name: Option<String>,
    pub(crate) artifact_id: Option<i64>,
    pub(crate) run_id: Option<i64>,
    pub(crate) artifact_created_at: Option<String>,
}

#[derive(Debug, Clone)]
pub(crate) struct ServerSummaryRow {
    pub(crate) tag_repo: String,
    pub(crate) tag_name: String,
    pub(crate) server_commit_sha: String,
    pub(crate) cliairplay_commits: Vec<String>,
    pub(crate) libraop_commits: Vec<String>,
    pub(crate) cliap2_binary_count: i64,
    pub(crate) cliraop_binary_count: i64,
    pub(crate) cliap2_match_count: i64,
    pub(crate) cliraop_match_count: i64,
}

#[derive(Debug, Clone)]
pub(crate) struct Stats {
    pub(crate) artifacts: i64,
    pub(crate) downloaded_artifacts: i64,
    pub(crate) expired_artifacts: i64,
    pub(crate) files: i64,
    pub(crate) repos: i64,
    pub(crate) commits: i64,
    pub(crate) server_tags: i64,
    pub(crate) server_tag_binaries: i64,
}

impl Database {
    pub(crate) async fn open(path: &Path) -> Result<Self> {
        let options = if path == Path::new(":memory:") {
            SqliteConnectOptions::from_str("sqlite::memory:")?
        } else {
            SqliteConnectOptions::new()
                .filename(path)
                .create_if_missing(true)
                .journal_mode(SqliteJournalMode::Wal)
                .busy_timeout(Duration::from_secs(30))
        };

        let pool = SqlitePoolOptions::new()
            .max_connections(5)
            .connect_with(options)
            .await?;

        let database = Self { pool };
        database.init().await?;
        Ok(database)
    }

    #[allow(clippy::too_many_lines)]
    async fn init(&self) -> Result<()> {
        sqlx::query("PRAGMA foreign_keys = ON")
            .execute(&self.pool)
            .await?;

        sqlx::query(
            r"
            CREATE TABLE IF NOT EXISTS artifacts (
              repo TEXT NOT NULL,
              artifact_id INTEGER NOT NULL,
              artifact_name TEXT NOT NULL,
              artifact_url TEXT NOT NULL,
              archive_download_url TEXT NOT NULL,
              artifact_digest TEXT,
              size_in_bytes INTEGER,
              expired INTEGER NOT NULL,
              created_at TEXT,
              expires_at TEXT,
              updated_at TEXT,
              run_id INTEGER,
              head_sha TEXT,
              head_branch TEXT,
              head_repository_id INTEGER,
              repository_id INTEGER,
              workflow_run_json TEXT,
              artifact_json TEXT NOT NULL,
              archive_sha256 TEXT,
              archive_size_bytes INTEGER,
              downloaded_at TEXT,
              http_status INTEGER,
              download_error TEXT,
              PRIMARY KEY (repo, artifact_id)
            )
            ",
        )
        .execute(&self.pool)
        .await?;

        sqlx::query(
            r"
            CREATE TABLE IF NOT EXISTS artifact_files (
              repo TEXT NOT NULL,
              artifact_id INTEGER NOT NULL,
              file_name TEXT NOT NULL,
              file_sha256 TEXT NOT NULL,
              file_size_bytes INTEGER NOT NULL,
              zip_crc32 INTEGER,
              zip_compress_size INTEGER,
              zip_file_size INTEGER,
              zip_date_time TEXT,
              zip_unix_mode INTEGER,
              PRIMARY KEY (repo, artifact_id, file_name),
              FOREIGN KEY (repo, artifact_id)
                REFERENCES artifacts(repo, artifact_id)
                ON DELETE CASCADE
            )
            ",
        )
        .execute(&self.pool)
        .await?;

        sqlx::query(
            r"
            CREATE TABLE IF NOT EXISTS server_tags (
              repo TEXT NOT NULL,
              tag_name TEXT NOT NULL,
              commit_sha TEXT NOT NULL,
              zipball_url TEXT NOT NULL,
              tarball_url TEXT NOT NULL,
              tag_json TEXT NOT NULL,
              source_zip_sha256 TEXT,
              source_zip_size_bytes INTEGER,
              crawled_at TEXT,
              downloaded_at TEXT,
              download_error TEXT,
              PRIMARY KEY (repo, tag_name)
            )
            ",
        )
        .execute(&self.pool)
        .await?;

        sqlx::query(
            r"
            CREATE TABLE IF NOT EXISTS server_tag_binaries (
              repo TEXT NOT NULL,
              tag_name TEXT NOT NULL,
              source_path TEXT NOT NULL,
              binary_name TEXT NOT NULL,
              helper_package TEXT NOT NULL,
              helper_repo TEXT NOT NULL,
              file_sha256 TEXT NOT NULL,
              file_size_bytes INTEGER NOT NULL,
              zip_crc32 INTEGER,
              zip_compress_size INTEGER,
              zip_file_size INTEGER,
              zip_date_time TEXT,
              zip_unix_mode INTEGER,
              PRIMARY KEY (repo, tag_name, source_path),
              FOREIGN KEY (repo, tag_name)
                REFERENCES server_tags(repo, tag_name)
                ON DELETE CASCADE
            )
            ",
        )
        .execute(&self.pool)
        .await?;

        let indices = [
            "CREATE INDEX IF NOT EXISTS artifact_files_sha256_idx ON artifact_files(file_sha256)",
            "CREATE INDEX IF NOT EXISTS artifacts_head_sha_idx ON artifacts(repo, head_sha)",
            "CREATE INDEX IF NOT EXISTS artifacts_name_idx ON artifacts(repo, artifact_name)",
            "CREATE INDEX IF NOT EXISTS artifacts_archive_sha256_idx ON artifacts(archive_sha256)",
            "CREATE INDEX IF NOT EXISTS server_tags_commit_idx ON server_tags(repo, commit_sha)",
            "CREATE INDEX IF NOT EXISTS server_tag_binaries_sha256_idx ON server_tag_binaries(file_sha256)",
            "CREATE INDEX IF NOT EXISTS server_tag_binaries_helper_idx ON server_tag_binaries(helper_repo, helper_package)",
        ];

        for statement in indices {
            sqlx::query(statement).execute(&self.pool).await?;
        }

        Ok(())
    }

    pub(crate) async fn upsert_artifact(&self, repo: &str, artifact: &Artifact) -> Result<()> {
        let workflow_run_json = match &artifact.workflow_run {
            Some(workflow_run) => Some(serde_json::to_string(workflow_run)?),
            None => None,
        };
        let artifact_json = serde_json::to_string(artifact)?;
        let workflow_run = artifact.workflow_run.as_ref();
        let expired = bool_to_i64(artifact.expired);

        sqlx::query(
            r"
            INSERT INTO artifacts (
              repo,
              artifact_id,
              artifact_name,
              artifact_url,
              archive_download_url,
              artifact_digest,
              size_in_bytes,
              expired,
              created_at,
              expires_at,
              updated_at,
              run_id,
              head_sha,
              head_branch,
              head_repository_id,
              repository_id,
              workflow_run_json,
              artifact_json
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            ON CONFLICT(repo, artifact_id) DO UPDATE SET
              artifact_name = excluded.artifact_name,
              artifact_url = excluded.artifact_url,
              archive_download_url = excluded.archive_download_url,
              artifact_digest = excluded.artifact_digest,
              size_in_bytes = excluded.size_in_bytes,
              expired = excluded.expired,
              created_at = excluded.created_at,
              expires_at = excluded.expires_at,
              updated_at = excluded.updated_at,
              run_id = excluded.run_id,
              head_sha = excluded.head_sha,
              head_branch = excluded.head_branch,
              head_repository_id = excluded.head_repository_id,
              repository_id = excluded.repository_id,
              workflow_run_json = excluded.workflow_run_json,
              artifact_json = excluded.artifact_json
            ",
        )
        .bind(repo)
        .bind(artifact.id)
        .bind(&artifact.name)
        .bind(&artifact.url)
        .bind(&artifact.archive_download_url)
        .bind(&artifact.digest)
        .bind(artifact.size_in_bytes)
        .bind(expired)
        .bind(&artifact.created_at)
        .bind(&artifact.expires_at)
        .bind(&artifact.updated_at)
        .bind(workflow_run.and_then(|run| run.id))
        .bind(workflow_run.and_then(|run| run.head_sha.as_deref()))
        .bind(workflow_run.and_then(|run| run.head_branch.as_deref()))
        .bind(workflow_run.and_then(|run| run.head_repository_id))
        .bind(workflow_run.and_then(|run| run.repository_id))
        .bind(workflow_run_json)
        .bind(artifact_json)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub(crate) async fn artifact_has_download(&self, repo: &str, artifact_id: i64) -> Result<bool> {
        let count: i64 = sqlx::query_scalar(
            r"
            SELECT COUNT(*)
            FROM artifacts
            WHERE repo = ? AND artifact_id = ? AND archive_sha256 IS NOT NULL
            ",
        )
        .bind(repo)
        .bind(artifact_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(count > 0)
    }

    pub(crate) async fn mark_downloaded(
        &self,
        repo: &str,
        artifact_id: i64,
        archive: &ArtifactArchive,
    ) -> Result<()> {
        sqlx::query(
            r"
            UPDATE artifacts
            SET archive_sha256 = ?,
                archive_size_bytes = ?,
                downloaded_at = strftime('%Y-%m-%dT%H:%M:%fZ', 'now'),
                expired = 0,
                http_status = 200,
                download_error = NULL
            WHERE repo = ? AND artifact_id = ?
            ",
        )
        .bind(&archive.archive_sha256)
        .bind(archive.archive_size_bytes)
        .bind(repo)
        .bind(artifact_id)
        .execute(&self.pool)
        .await?;

        sqlx::query("DELETE FROM artifact_files WHERE repo = ? AND artifact_id = ?")
            .bind(repo)
            .bind(artifact_id)
            .execute(&self.pool)
            .await?;

        for file in &archive.files {
            self.insert_artifact_file(repo, artifact_id, file).await?;
        }

        Ok(())
    }

    pub(crate) async fn mark_gone(&self, repo: &str, artifact_id: i64) -> Result<()> {
        sqlx::query(
            r"
            UPDATE artifacts
            SET expired = 1,
                http_status = 410,
                download_error = 'artifact gone'
            WHERE repo = ? AND artifact_id = ?
            ",
        )
        .bind(repo)
        .bind(artifact_id)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub(crate) async fn mark_download_error(
        &self,
        repo: &str,
        artifact_id: i64,
        error: &str,
    ) -> Result<()> {
        sqlx::query(
            r"
            UPDATE artifacts
            SET download_error = ?
            WHERE repo = ? AND artifact_id = ?
            ",
        )
        .bind(error)
        .bind(repo)
        .bind(artifact_id)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub(crate) async fn upsert_server_tag_metadata(&self, repo: &str, tag: &Tag) -> Result<()> {
        let tag_json = serde_json::to_string(tag)?;
        sqlx::query(
            r"
            INSERT INTO server_tags (
              repo,
              tag_name,
              commit_sha,
              zipball_url,
              tarball_url,
              tag_json,
              crawled_at
            ) VALUES (?, ?, ?, ?, ?, ?, strftime('%Y-%m-%dT%H:%M:%fZ', 'now'))
            ON CONFLICT(repo, tag_name) DO UPDATE SET
              commit_sha = excluded.commit_sha,
              zipball_url = excluded.zipball_url,
              tarball_url = excluded.tarball_url,
              tag_json = excluded.tag_json,
              crawled_at = excluded.crawled_at
            ",
        )
        .bind(repo)
        .bind(&tag.name)
        .bind(&tag.commit.sha)
        .bind(&tag.zipball_url)
        .bind(&tag.tarball_url)
        .bind(tag_json)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub(crate) async fn server_tag_has_download(&self, repo: &str, tag_name: &str) -> Result<bool> {
        let count: i64 = sqlx::query_scalar(
            r"
            SELECT COUNT(*)
            FROM server_tags
            WHERE repo = ? AND tag_name = ? AND source_zip_sha256 IS NOT NULL
            ",
        )
        .bind(repo)
        .bind(tag_name)
        .fetch_one(&self.pool)
        .await?;

        Ok(count > 0)
    }

    pub(crate) async fn mark_server_tag_downloaded(
        &self,
        repo: &str,
        tag: &Tag,
        source_archive: &ServerSourceArchive,
    ) -> Result<()> {
        self.upsert_server_tag_metadata(repo, tag).await?;

        sqlx::query(
            r"
            UPDATE server_tags
            SET source_zip_sha256 = ?,
                source_zip_size_bytes = ?,
                downloaded_at = strftime('%Y-%m-%dT%H:%M:%fZ', 'now'),
                download_error = NULL
            WHERE repo = ? AND tag_name = ?
            ",
        )
        .bind(&source_archive.source_zip_sha256)
        .bind(source_archive.source_zip_size_bytes)
        .bind(repo)
        .bind(&tag.name)
        .execute(&self.pool)
        .await?;

        sqlx::query("DELETE FROM server_tag_binaries WHERE repo = ? AND tag_name = ?")
            .bind(repo)
            .bind(&tag.name)
            .execute(&self.pool)
            .await?;

        for binary in &source_archive.binaries {
            self.insert_server_binary(repo, &tag.name, binary).await?;
        }

        Ok(())
    }

    pub(crate) async fn mark_server_tag_error(
        &self,
        repo: &str,
        tag_name: &str,
        error: &str,
    ) -> Result<()> {
        sqlx::query(
            r"
            UPDATE server_tags
            SET download_error = ?
            WHERE repo = ? AND tag_name = ?
            ",
        )
        .bind(error)
        .bind(repo)
        .bind(tag_name)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub(crate) async fn query_file_hash(&self, file_sha256: &str) -> Result<Vec<FileMatch>> {
        let rows = sqlx::query(
            r"
            SELECT
              a.repo,
              a.artifact_id,
              a.artifact_name,
              a.artifact_digest,
              a.archive_sha256,
              a.archive_size_bytes,
              f.file_name,
              f.file_sha256,
              f.file_size_bytes,
              a.run_id,
              a.head_sha,
              a.head_branch,
              a.created_at,
              a.expires_at,
              a.expired
            FROM artifact_files f
            JOIN artifacts a
              ON a.repo = f.repo AND a.artifact_id = f.artifact_id
            WHERE f.file_sha256 = ?
            ORDER BY a.created_at DESC, a.repo ASC, a.artifact_name ASC
            ",
        )
        .bind(file_sha256)
        .fetch_all(&self.pool)
        .await?;

        rows.into_iter()
            .map(|row| row_to_file_match(&row))
            .collect()
    }

    pub(crate) async fn query_archive_hash(
        &self,
        archive_sha256: &str,
    ) -> Result<Vec<ArtifactMatch>> {
        let rows = sqlx::query(
            r"
            SELECT
              repo,
              artifact_id,
              artifact_name,
              artifact_digest,
              archive_sha256,
              archive_size_bytes,
              run_id,
              head_sha,
              head_branch,
              created_at,
              expires_at,
              expired
            FROM artifacts
            WHERE archive_sha256 = ? OR artifact_digest = ? OR artifact_digest = ?
            ORDER BY created_at DESC, repo ASC, artifact_name ASC
            ",
        )
        .bind(archive_sha256)
        .bind(archive_sha256)
        .bind(format!("sha256:{archive_sha256}"))
        .fetch_all(&self.pool)
        .await?;

        rows.into_iter()
            .map(|row| row_to_artifact_match(&row))
            .collect()
    }

    pub(crate) async fn list_files(
        &self,
        repo: Option<&str>,
        head_sha: Option<&str>,
        artifact_name: Option<&str>,
        limit: i64,
    ) -> Result<Vec<FileMatch>> {
        let mut builder = QueryBuilder::<Sqlite>::new(
            r"
            SELECT
              a.repo,
              a.artifact_id,
              a.artifact_name,
              a.artifact_digest,
              a.archive_sha256,
              a.archive_size_bytes,
              f.file_name,
              f.file_sha256,
              f.file_size_bytes,
              a.run_id,
              a.head_sha,
              a.head_branch,
              a.created_at,
              a.expires_at,
              a.expired
            FROM artifact_files f
            JOIN artifacts a
              ON a.repo = f.repo AND a.artifact_id = f.artifact_id
            WHERE 1 = 1
            ",
        );

        if let Some(repo) = repo {
            builder.push(" AND a.repo = ").push_bind(repo);
        }
        if let Some(head_sha) = head_sha {
            builder.push(" AND a.head_sha = ").push_bind(head_sha);
        }
        if let Some(artifact_name) = artifact_name {
            builder
                .push(" AND a.artifact_name = ")
                .push_bind(artifact_name);
        }

        builder.push(" ORDER BY a.created_at DESC, a.repo ASC, a.artifact_name ASC LIMIT ");
        builder.push_bind(limit);

        let rows = builder.build().fetch_all(&self.pool).await?;
        rows.into_iter()
            .map(|row| row_to_file_match(&row))
            .collect()
    }

    pub(crate) async fn resolve_server_tag(
        &self,
        repo: &str,
        tag_name: &str,
    ) -> Result<Vec<ServerTagResolution>> {
        let rows = sqlx::query(
            r"
            SELECT
              st.repo AS tag_repo,
              st.tag_name,
              st.commit_sha AS server_commit_sha,
              stb.binary_name,
              stb.helper_package,
              stb.helper_repo,
              stb.file_sha256 AS binary_sha256,
              a.head_sha AS helper_commit,
              a.artifact_name,
              a.artifact_id,
              a.run_id,
              a.created_at AS artifact_created_at
            FROM server_tag_binaries stb
            JOIN server_tags st
              ON st.repo = stb.repo AND st.tag_name = stb.tag_name
            LEFT JOIN artifact_files f
              ON f.file_sha256 = stb.file_sha256 AND f.repo = stb.helper_repo
            LEFT JOIN artifacts a
              ON a.repo = f.repo AND a.artifact_id = f.artifact_id
            WHERE st.repo = ? AND st.tag_name = ?
            ORDER BY stb.helper_package ASC, stb.binary_name ASC, a.created_at DESC
            ",
        )
        .bind(repo)
        .bind(tag_name)
        .fetch_all(&self.pool)
        .await?;

        rows.into_iter()
            .map(|row| row_to_server_tag_resolution(&row))
            .collect()
    }

    pub(crate) async fn server_summary_rows(&self, repo: &str) -> Result<Vec<ServerSummaryRow>> {
        let rows = sqlx::query(
            r"
            SELECT
              st.repo AS tag_repo,
              st.tag_name,
              st.commit_sha AS server_commit_sha,
              COALESCE((
                SELECT GROUP_CONCAT(DISTINCT a.head_sha)
                FROM server_tag_binaries b
                JOIN artifact_files f
                  ON f.file_sha256 = b.file_sha256 AND f.repo = b.helper_repo
                JOIN artifacts a
                  ON a.repo = f.repo AND a.artifact_id = f.artifact_id
                WHERE b.repo = st.repo
                  AND b.tag_name = st.tag_name
                  AND b.helper_package = 'cliairplay'
                  AND a.head_sha IS NOT NULL
              ), '') AS cliairplay_commits,
              COALESCE((
                SELECT GROUP_CONCAT(DISTINCT a.head_sha)
                FROM server_tag_binaries b
                JOIN artifact_files f
                  ON f.file_sha256 = b.file_sha256 AND f.repo = b.helper_repo
                JOIN artifacts a
                  ON a.repo = f.repo AND a.artifact_id = f.artifact_id
                WHERE b.repo = st.repo
                  AND b.tag_name = st.tag_name
                  AND b.helper_package = 'libraop'
                  AND a.head_sha IS NOT NULL
              ), '') AS libraop_commits,
              COALESCE((
                SELECT COUNT(DISTINCT b.file_sha256)
                FROM server_tag_binaries b
                WHERE b.repo = st.repo
                  AND b.tag_name = st.tag_name
                  AND b.helper_package = 'cliairplay'
              ), 0) AS cliap2_binary_count,
              COALESCE((
                SELECT COUNT(DISTINCT b.file_sha256)
                FROM server_tag_binaries b
                WHERE b.repo = st.repo
                  AND b.tag_name = st.tag_name
                  AND b.helper_package = 'libraop'
              ), 0) AS cliraop_binary_count,
              COALESCE((
                SELECT COUNT(DISTINCT b.file_sha256)
                FROM server_tag_binaries b
                JOIN artifact_files f
                  ON f.file_sha256 = b.file_sha256 AND f.repo = b.helper_repo
                WHERE b.repo = st.repo
                  AND b.tag_name = st.tag_name
                  AND b.helper_package = 'cliairplay'
              ), 0) AS cliap2_match_count,
              COALESCE((
                SELECT COUNT(DISTINCT b.file_sha256)
                FROM server_tag_binaries b
                JOIN artifact_files f
                  ON f.file_sha256 = b.file_sha256 AND f.repo = b.helper_repo
                WHERE b.repo = st.repo
                  AND b.tag_name = st.tag_name
                  AND b.helper_package = 'libraop'
              ), 0) AS cliraop_match_count
            FROM server_tags st
            WHERE st.repo = ?
            ",
        )
        .bind(repo)
        .fetch_all(&self.pool)
        .await?;

        rows.into_iter()
            .map(|row| row_to_server_summary(&row))
            .collect()
    }

    pub(crate) async fn stats(&self) -> Result<Stats> {
        let row = sqlx::query(
            r"
            SELECT
              (SELECT COUNT(*) FROM artifacts) AS artifacts,
              (SELECT COUNT(*) FROM artifacts WHERE archive_sha256 IS NOT NULL) AS downloaded_artifacts,
              (SELECT COUNT(*) FROM artifacts WHERE expired = 1) AS expired_artifacts,
              (SELECT COUNT(*) FROM artifact_files) AS files,
              (SELECT COUNT(DISTINCT repo) FROM artifacts) AS repos,
              (SELECT COUNT(DISTINCT repo || ':' || COALESCE(head_sha, '')) FROM artifacts WHERE head_sha IS NOT NULL) AS commits,
              (SELECT COUNT(*) FROM server_tags) AS server_tags,
              (SELECT COUNT(*) FROM server_tag_binaries) AS server_tag_binaries
            ",
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(Stats {
            artifacts: row.try_get("artifacts")?,
            downloaded_artifacts: row.try_get("downloaded_artifacts")?,
            expired_artifacts: row.try_get("expired_artifacts")?,
            files: row.try_get("files")?,
            repos: row.try_get("repos")?,
            commits: row.try_get("commits")?,
            server_tags: row.try_get("server_tags")?,
            server_tag_binaries: row.try_get("server_tag_binaries")?,
        })
    }

    async fn insert_artifact_file(
        &self,
        repo: &str,
        artifact_id: i64,
        file: &ArtifactFile,
    ) -> Result<()> {
        sqlx::query(
            r"
            INSERT INTO artifact_files (
              repo,
              artifact_id,
              file_name,
              file_sha256,
              file_size_bytes,
              zip_crc32,
              zip_compress_size,
              zip_file_size,
              zip_date_time,
              zip_unix_mode
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            ON CONFLICT(repo, artifact_id, file_name) DO UPDATE SET
              file_sha256 = excluded.file_sha256,
              file_size_bytes = excluded.file_size_bytes,
              zip_crc32 = excluded.zip_crc32,
              zip_compress_size = excluded.zip_compress_size,
              zip_file_size = excluded.zip_file_size,
              zip_date_time = excluded.zip_date_time,
              zip_unix_mode = excluded.zip_unix_mode
            ",
        )
        .bind(repo)
        .bind(artifact_id)
        .bind(&file.file_name)
        .bind(&file.file_sha256)
        .bind(file.file_size_bytes)
        .bind(file.zip_crc32)
        .bind(file.zip_compress_size)
        .bind(file.zip_file_size)
        .bind(&file.zip_date_time)
        .bind(file.zip_unix_mode)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn insert_server_binary(
        &self,
        repo: &str,
        tag_name: &str,
        binary: &ServerBinary,
    ) -> Result<()> {
        sqlx::query(
            r"
            INSERT INTO server_tag_binaries (
              repo,
              tag_name,
              source_path,
              binary_name,
              helper_package,
              helper_repo,
              file_sha256,
              file_size_bytes,
              zip_crc32,
              zip_compress_size,
              zip_file_size,
              zip_date_time,
              zip_unix_mode
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            ON CONFLICT(repo, tag_name, source_path) DO UPDATE SET
              binary_name = excluded.binary_name,
              helper_package = excluded.helper_package,
              helper_repo = excluded.helper_repo,
              file_sha256 = excluded.file_sha256,
              file_size_bytes = excluded.file_size_bytes,
              zip_crc32 = excluded.zip_crc32,
              zip_compress_size = excluded.zip_compress_size,
              zip_file_size = excluded.zip_file_size,
              zip_date_time = excluded.zip_date_time,
              zip_unix_mode = excluded.zip_unix_mode
            ",
        )
        .bind(repo)
        .bind(tag_name)
        .bind(&binary.source_path)
        .bind(&binary.binary_name)
        .bind(&binary.helper_package)
        .bind(&binary.helper_repo)
        .bind(&binary.file_sha256)
        .bind(binary.file_size_bytes)
        .bind(binary.zip_crc32)
        .bind(binary.zip_compress_size)
        .bind(binary.zip_file_size)
        .bind(&binary.zip_date_time)
        .bind(binary.zip_unix_mode)
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}

fn row_to_file_match(row: &sqlx::sqlite::SqliteRow) -> Result<FileMatch> {
    Ok(FileMatch {
        repo: row.try_get("repo")?,
        artifact_id: row.try_get("artifact_id")?,
        artifact_name: row.try_get("artifact_name")?,
        artifact_digest: row.try_get("artifact_digest")?,
        archive_sha256: row.try_get("archive_sha256")?,
        archive_size_bytes: row.try_get("archive_size_bytes")?,
        file_name: row.try_get("file_name")?,
        file_sha256: row.try_get("file_sha256")?,
        file_size_bytes: row.try_get("file_size_bytes")?,
        run_id: row.try_get("run_id")?,
        head_sha: row.try_get("head_sha")?,
        head_branch: row.try_get("head_branch")?,
        created_at: row.try_get("created_at")?,
        expires_at: row.try_get("expires_at")?,
        expired: int_to_bool(row.try_get::<i64, _>("expired")?),
    })
}

fn row_to_artifact_match(row: &sqlx::sqlite::SqliteRow) -> Result<ArtifactMatch> {
    Ok(ArtifactMatch {
        repo: row.try_get("repo")?,
        artifact_id: row.try_get("artifact_id")?,
        artifact_name: row.try_get("artifact_name")?,
        artifact_digest: row.try_get("artifact_digest")?,
        archive_sha256: row.try_get("archive_sha256")?,
        archive_size_bytes: row.try_get("archive_size_bytes")?,
        run_id: row.try_get("run_id")?,
        head_sha: row.try_get("head_sha")?,
        head_branch: row.try_get("head_branch")?,
        created_at: row.try_get("created_at")?,
        expires_at: row.try_get("expires_at")?,
        expired: int_to_bool(row.try_get::<i64, _>("expired")?),
    })
}

fn row_to_server_tag_resolution(row: &sqlx::sqlite::SqliteRow) -> Result<ServerTagResolution> {
    Ok(ServerTagResolution {
        tag_repo: row.try_get("tag_repo")?,
        tag_name: row.try_get("tag_name")?,
        server_commit_sha: row.try_get("server_commit_sha")?,
        binary_name: row.try_get("binary_name")?,
        helper_package: row.try_get("helper_package")?,
        helper_repo: row.try_get("helper_repo")?,
        binary_sha256: row.try_get("binary_sha256")?,
        helper_commit: row.try_get("helper_commit")?,
        artifact_name: row.try_get("artifact_name")?,
        artifact_id: row.try_get("artifact_id")?,
        run_id: row.try_get("run_id")?,
        artifact_created_at: row.try_get("artifact_created_at")?,
    })
}

fn row_to_server_summary(row: &sqlx::sqlite::SqliteRow) -> Result<ServerSummaryRow> {
    let cliairplay_commits: String = row.try_get("cliairplay_commits")?;
    let libraop_commits: String = row.try_get("libraop_commits")?;

    Ok(ServerSummaryRow {
        tag_repo: row.try_get("tag_repo")?,
        tag_name: row.try_get("tag_name")?,
        server_commit_sha: row.try_get("server_commit_sha")?,
        cliairplay_commits: split_commits(&cliairplay_commits),
        libraop_commits: split_commits(&libraop_commits),
        cliap2_binary_count: row.try_get("cliap2_binary_count")?,
        cliraop_binary_count: row.try_get("cliraop_binary_count")?,
        cliap2_match_count: row.try_get("cliap2_match_count")?,
        cliraop_match_count: row.try_get("cliraop_match_count")?,
    })
}

fn split_commits(value: &str) -> Vec<String> {
    value
        .split(',')
        .filter(|part| !part.is_empty())
        .map(ToOwned::to_owned)
        .collect()
}

const fn bool_to_i64(value: bool) -> i64 {
    if value { 1 } else { 0 }
}

const fn int_to_bool(value: i64) -> bool {
    value != 0
}
