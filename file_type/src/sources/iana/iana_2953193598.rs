use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_2953193598: FileType = FileType {
    file_format: &FileFormat {
        id: 2_953_193_598,
        source_type: SourceType::Iana,
        name: "yang-data+json",
        extensions: &[],
        media_types: &["application/yang-data+json"],
        signatures: &[],
        related_formats: &[],
    },
};
