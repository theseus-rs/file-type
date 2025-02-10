use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1767270444: FileType = FileType {
    file_format: &FileFormat {
        id: 1_767_270_444,
        source_type: SourceType::Iana,
        name: "vnd.drive+json",
        extensions: &[],
        media_types: &["application/vnd.drive+json"],
        signatures: &[],
        related_formats: &[],
    },
};
