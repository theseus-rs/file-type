use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_905371884: FileType = FileType {
    file_format: &FileFormat {
        id: 905_371_884,
        source_type: SourceType::Linguist,
        name: "jq",
        extensions: &["jq"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
