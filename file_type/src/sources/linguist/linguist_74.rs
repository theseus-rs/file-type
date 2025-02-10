use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_74: FileType = FileType {
    file_format: &FileFormat {
        id: 74,
        source_type: SourceType::Linguist,
        name: "Csound Document",
        extensions: &["csd"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
