use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_1054258749: FileType = FileType {
    file_format: &FileFormat {
        id: 1_054_258_749,
        source_type: SourceType::Linguist,
        name: "Gleam",
        extensions: &["gleam"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
