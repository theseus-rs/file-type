use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_3921585630: FileType = FileType {
    file_format: &FileFormat {
        id: 3_921_585_630,
        source_type: SourceType::Iana,
        name: "vnd.mpegurl",
        extensions: &[],
        media_types: &["video/vnd.mpegurl"],
        signatures: &[],
        related_formats: &[],
    },
};
