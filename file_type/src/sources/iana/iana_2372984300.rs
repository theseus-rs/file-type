use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_2372984300: FileType = FileType {
    file_format: &FileFormat {
        id: 2_372_984_300,
        source_type: SourceType::Iana,
        name: "json-patch+json",
        extensions: &[],
        media_types: &["application/json-patch+json"],
        signatures: &[],
        related_formats: &[],
    },
};
