//! Storage Format Definitions
//!
//! Defines the binary storage format and compression options.

use serde::{Deserialize, Serialize};

/// Storage format type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum StorageFormat {
    /// JSON Lines format (one JSON object per line)
    #[default]
    JsonLines,
    /// Binary format (more compact)
    Binary,
    /// MessagePack format
    MessagePack,
}

impl std::fmt::Display for StorageFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StorageFormat::JsonLines => write!(f, "jsonl"),
            StorageFormat::Binary => write!(f, "bin"),
            StorageFormat::MessagePack => write!(f, "msgpack"),
        }
    }
}

/// Compression type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum CompressionType {
    /// No compression
    #[default]
    None,
    /// Gzip compression
    Gzip,
    /// Zstd compression (better compression ratio)
    Zstd,
}

impl std::fmt::Display for CompressionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CompressionType::None => write!(f, "none"),
            CompressionType::Gzip => write!(f, "gzip"),
            CompressionType::Zstd => write!(f, "zstd"),
        }
    }
}

/// File extension based on format and compression
pub fn get_file_extension(format: StorageFormat, compression: CompressionType) -> String {
    let format_ext = match format {
        StorageFormat::JsonLines => "jsonl",
        StorageFormat::Binary => "bin",
        StorageFormat::MessagePack => "msgpack",
    };

    let compression_ext = match compression {
        CompressionType::None => "",
        CompressionType::Gzip => ".gz",
        CompressionType::Zstd => ".zst",
    };

    format!("{}{}", format_ext, compression_ext)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_storage_format_display() {
        assert_eq!(StorageFormat::JsonLines.to_string(), "jsonl");
        assert_eq!(StorageFormat::Binary.to_string(), "bin");
        assert_eq!(StorageFormat::MessagePack.to_string(), "msgpack");
    }

    #[test]
    fn test_compression_type_display() {
        assert_eq!(CompressionType::None.to_string(), "none");
        assert_eq!(CompressionType::Gzip.to_string(), "gzip");
        assert_eq!(CompressionType::Zstd.to_string(), "zstd");
    }

    #[test]
    fn test_file_extension() {
        assert_eq!(
            get_file_extension(StorageFormat::JsonLines, CompressionType::None),
            "jsonl"
        );
        assert_eq!(
            get_file_extension(StorageFormat::JsonLines, CompressionType::Gzip),
            "jsonl.gz"
        );
        assert_eq!(
            get_file_extension(StorageFormat::Binary, CompressionType::Zstd),
            "bin.zst"
        );
    }

    #[test]
    fn test_defaults() {
        assert_eq!(StorageFormat::default(), StorageFormat::JsonLines);
        assert_eq!(CompressionType::default(), CompressionType::None);
    }
}
