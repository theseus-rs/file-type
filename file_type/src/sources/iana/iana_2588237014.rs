use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2588237014: FileType = FileType {
    file_format: &FileFormat {
        id: 2_588_237_014,
        source_type: SourceType::Iana,
        name: "yang-data+xml",
        extensions: &[],
        media_types: &["application/yang-data+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
