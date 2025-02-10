use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_3382036996: FileType = FileType {
    file_format: &FileFormat {
        id: 3_382_036_996,
        source_type: SourceType::Iana,
        name: "vnd.oma.cab-address-book+xml",
        extensions: &[],
        media_types: &["application/vnd.oma.cab-address-book+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
