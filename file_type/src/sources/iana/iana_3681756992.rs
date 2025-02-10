use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_3681756992: FileType = FileType {
    file_format: &FileFormat {
        id: 3_681_756_992,
        source_type: SourceType::Iana,
        name: "lostsync+xml",
        extensions: &[],
        media_types: &["application/lostsync+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
