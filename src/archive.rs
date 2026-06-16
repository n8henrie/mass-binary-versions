use std::io::{Cursor, Read};

use zip::ZipArchive;

use crate::error::{AppError, Result};
use crate::hash::sha256_hex_bytes;

const AIRPLAY_BIN_DIR: &str = "music_assistant/providers/airplay/bin/";

#[derive(Debug, Clone)]
pub(crate) struct ArtifactArchive {
    pub(crate) archive_sha256: String,
    pub(crate) archive_size_bytes: i64,
    pub(crate) files: Vec<ArtifactFile>,
}

#[derive(Debug, Clone)]
pub(crate) struct ArtifactFile {
    pub(crate) file_name: String,
    pub(crate) file_sha256: String,
    pub(crate) file_size_bytes: i64,
    pub(crate) zip_crc32: Option<i64>,
    pub(crate) zip_compress_size: Option<i64>,
    pub(crate) zip_file_size: Option<i64>,
    pub(crate) zip_date_time: Option<String>,
    pub(crate) zip_unix_mode: Option<i64>,
}

#[derive(Debug, Clone)]
pub(crate) struct ServerSourceArchive {
    pub(crate) source_zip_sha256: String,
    pub(crate) source_zip_size_bytes: i64,
    pub(crate) binaries: Vec<ServerBinary>,
}

#[derive(Debug, Clone)]
pub(crate) struct ServerBinary {
    pub(crate) source_path: String,
    pub(crate) binary_name: String,
    pub(crate) helper_package: String,
    pub(crate) helper_repo: String,
    pub(crate) file_sha256: String,
    pub(crate) file_size_bytes: i64,
    pub(crate) zip_crc32: Option<i64>,
    pub(crate) zip_compress_size: Option<i64>,
    pub(crate) zip_file_size: Option<i64>,
    pub(crate) zip_date_time: Option<String>,
    pub(crate) zip_unix_mode: Option<i64>,
}

pub(crate) fn inspect_downloaded_artifact(
    repo: &str,
    artifact_id: i64,
    artifact_name: &str,
    bytes: &[u8],
) -> Result<ArtifactArchive> {
    let archive_size_bytes =
        i64::try_from(bytes.len()).map_err(|_error| AppError::ArtifactTooLarge {
            repo: repo.to_owned(),
            artifact_id,
        })?;
    let archive_sha256 = sha256_hex_bytes(bytes);

    if looks_like_zip(bytes) {
        let files = inspect_zip(bytes)?;
        if files.is_empty() {
            return Err(AppError::EmptyArtifact {
                repo: repo.to_owned(),
                artifact_id,
            });
        }
        Ok(ArtifactArchive {
            archive_sha256,
            archive_size_bytes,
            files,
        })
    } else {
        let file_size_bytes = archive_size_bytes;
        Ok(ArtifactArchive {
            archive_sha256: archive_sha256.clone(),
            archive_size_bytes,
            files: vec![ArtifactFile {
                file_name: artifact_name.to_owned(),
                file_sha256: archive_sha256,
                file_size_bytes,
                zip_crc32: None,
                zip_compress_size: None,
                zip_file_size: None,
                zip_date_time: None,
                zip_unix_mode: None,
            }],
        })
    }
}

pub(crate) fn inspect_server_source_zip(
    repo: &str,
    tag_name: &str,
    bytes: &[u8],
) -> Result<ServerSourceArchive> {
    let source_zip_size_bytes =
        i64::try_from(bytes.len()).map_err(|_error| AppError::SourceZipTooLarge {
            repo: repo.to_owned(),
            tag_name: tag_name.to_owned(),
        })?;
    let source_zip_sha256 = sha256_hex_bytes(bytes);

    if !looks_like_zip(bytes) {
        return Err(AppError::NotZipball {
            repo: repo.to_owned(),
            tag_name: tag_name.to_owned(),
        });
    }

    let cursor = Cursor::new(bytes);
    let mut archive = ZipArchive::new(cursor)?;
    let mut binaries = Vec::new();

    for index in 0..archive.len() {
        let mut zip_file = archive.by_index(index)?;
        if zip_file.is_dir() {
            continue;
        }

        let source_path = normalized_zipball_path(zip_file.name());
        let Some(binary_name) = source_path
            .strip_prefix(AIRPLAY_BIN_DIR)
            .map(ToOwned::to_owned)
        else {
            continue;
        };
        if binary_name.contains('/') {
            continue;
        }
        let Some((helper_package, helper_repo)) = helper_for_binary(&binary_name) else {
            continue;
        };

        let zip_crc32 = Some(i64::from(zip_file.crc32()));
        let zip_compress_size = Some(u64_to_i64(zip_file.compressed_size())?);
        let zip_file_size = Some(u64_to_i64(zip_file.size())?);
        let zip_date_time = Some(format_zip_date_time(zip_file.last_modified()));
        let zip_unix_mode = zip_file.unix_mode().map(i64::from);

        let mut contents = Vec::new();
        zip_file.read_to_end(&mut contents)?;
        let file_size_bytes =
            i64::try_from(contents.len()).map_err(|_error| AppError::SourceZipTooLarge {
                repo: repo.to_owned(),
                tag_name: tag_name.to_owned(),
            })?;

        binaries.push(ServerBinary {
            source_path,
            binary_name,
            helper_package: helper_package.to_owned(),
            helper_repo: helper_repo.to_owned(),
            file_sha256: sha256_hex_bytes(&contents),
            file_size_bytes,
            zip_crc32,
            zip_compress_size,
            zip_file_size,
            zip_date_time,
            zip_unix_mode,
        });
    }

    Ok(ServerSourceArchive {
        source_zip_sha256,
        source_zip_size_bytes,
        binaries,
    })
}

fn inspect_zip(bytes: &[u8]) -> Result<Vec<ArtifactFile>> {
    let cursor = Cursor::new(bytes);
    let mut archive = ZipArchive::new(cursor)?;
    let mut files = Vec::with_capacity(archive.len());

    for index in 0..archive.len() {
        let mut zip_file = archive.by_index(index)?;
        if zip_file.is_dir() {
            continue;
        }

        let file_name = match zip_file.enclosed_name() {
            Some(path) => path.to_string_lossy().into_owned(),
            None => zip_file.name().to_owned(),
        };
        let zip_crc32 = Some(i64::from(zip_file.crc32()));
        let zip_compress_size = Some(u64_to_i64(zip_file.compressed_size())?);
        let zip_file_size = Some(u64_to_i64(zip_file.size())?);
        let zip_date_time = Some(format_zip_date_time(zip_file.last_modified()));
        let zip_unix_mode = zip_file.unix_mode().map(i64::from);

        let mut contents = Vec::new();
        zip_file.read_to_end(&mut contents)?;

        files.push(ArtifactFile {
            file_name,
            file_sha256: sha256_hex_bytes(&contents),
            file_size_bytes: i64::try_from(contents.len()).map_err(|_error| {
                AppError::ArtifactTooLarge {
                    repo: String::from("zip-entry"),
                    artifact_id: 0,
                }
            })?,
            zip_crc32,
            zip_compress_size,
            zip_file_size,
            zip_date_time,
            zip_unix_mode,
        });
    }

    Ok(files)
}

fn normalized_zipball_path(path: &str) -> String {
    let mut parts = path.splitn(2, '/');
    let _root = parts.next();
    match parts.next() {
        Some(rest) => rest.to_owned(),
        None => path.to_owned(),
    }
}

fn helper_for_binary(binary_name: &str) -> Option<(&'static str, &'static str)> {
    if binary_name.starts_with("cliap2-") {
        Some(("cliairplay", "music-assistant/cliairplay"))
    } else if binary_name.starts_with("cliraop-") {
        Some(("libraop", "music-assistant/libraop"))
    } else {
        None
    }
}

fn looks_like_zip(bytes: &[u8]) -> bool {
    bytes.starts_with(b"PK\x03\x04")
        || bytes.starts_with(b"PK\x05\x06")
        || bytes.starts_with(b"PK\x07\x08")
}

fn u64_to_i64(value: u64) -> Result<i64> {
    i64::try_from(value).map_err(|_error| AppError::ArtifactTooLarge {
        repo: String::from("zip-entry"),
        artifact_id: 0,
    })
}

fn format_zip_date_time(value: zip::DateTime) -> String {
    format!(
        "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}",
        value.year(),
        value.month(),
        value.day(),
        value.hour(),
        value.minute(),
        value.second()
    )
}

#[cfg(test)]
mod tests {
    use super::{helper_for_binary, looks_like_zip, normalized_zipball_path};

    #[test]
    fn detects_zip_magic() {
        assert!(looks_like_zip(b"PK\x03\x04rest"));
        assert!(!looks_like_zip(b"not a zip"));
    }

    #[test]
    fn strips_zipball_root() {
        assert_eq!(
            normalized_zipball_path(
                "music-assistant-server-abc123/music_assistant/providers/airplay/bin/cliraop-linux-x86_64"
            ),
            "music_assistant/providers/airplay/bin/cliraop-linux-x86_64"
        );
    }

    #[test]
    fn maps_helper_binaries() {
        assert_eq!(
            helper_for_binary("cliap2-linux-x86_64"),
            Some(("cliairplay", "music-assistant/cliairplay"))
        );
        assert_eq!(
            helper_for_binary("cliraop-linux-x86_64"),
            Some(("libraop", "music-assistant/libraop"))
        );
        assert!(helper_for_binary("other").is_none());
    }
}
