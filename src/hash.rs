use sha2::{Digest, Sha256};
use std::path::Path;
use tokio::io::AsyncReadExt;

use crate::error::{AppError, Result};

pub(crate) fn sha256_hex_bytes(bytes: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    lower_hex(&hasher.finalize())
}

pub(crate) async fn sha256_hex_file(path: &Path) -> Result<String> {
    let mut file = tokio::fs::File::open(path).await?;
    let mut hasher = Sha256::new();
    let mut buffer = vec![0_u8; 64 * 1024].into_boxed_slice();

    loop {
        let bytes_read = file.read(&mut buffer).await?;
        if bytes_read == 0 {
            break;
        }
        hasher.update(&buffer[..bytes_read]);
    }

    Ok(lower_hex(&hasher.finalize()))
}

pub(crate) fn normalize_sha256(value: &str) -> Result<String> {
    let trimmed = value.trim();
    let without_prefix = match trimmed.strip_prefix("sha256:") {
        Some(value) => value,
        None => trimmed,
    };
    let normalized = without_prefix.to_ascii_lowercase();

    if normalized.len() == 64 && normalized.bytes().all(|b| b.is_ascii_hexdigit()) {
        Ok(normalized)
    } else {
        Err(AppError::InvalidSha256(value.to_owned()))
    }
}

fn lower_hex(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut output = String::with_capacity(bytes.len() * 2);

    for byte in bytes {
        let high = usize::from(byte >> 4);
        let low = usize::from(byte & 0x0f);
        output.push(char::from(HEX[high]));
        output.push(char::from(HEX[low]));
    }

    output
}

#[cfg(test)]
mod tests {
    use super::{normalize_sha256, sha256_hex_bytes};

    #[test]
    fn hashes_empty_bytes() {
        assert_eq!(
            sha256_hex_bytes(&[]),
            "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
        );
    }

    #[test]
    fn normalizes_sha256_prefix() {
        let value = "sha256:E3B0C44298FC1C149AFBF4C8996FB92427AE41E4649B934CA495991B7852B855";
        match normalize_sha256(value) {
            Ok(normalized) => assert_eq!(
                normalized,
                "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
            ),
            Err(error) => panic!("valid sha256 should normalize: {error}"),
        }
    }

    #[test]
    fn rejects_bad_sha256() {
        assert!(normalize_sha256("not a sha").is_err());
    }
}
