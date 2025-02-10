use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_2608284529: FileType = FileType {
    file_format: &FileFormat {
        id: 2_608_284_529,
        source_type: SourceType::Iana,
        name: "yang-patch+json",
        extensions: &[],
        media_types: &["application/yang-patch+json"],
        signatures: &[],
        related_formats: &[],
    },
};
