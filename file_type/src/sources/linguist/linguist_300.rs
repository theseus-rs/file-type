use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_300: FileType = FileType {
    file_format: &FileFormat {
        id: 300,
        source_type: SourceType::Linguist,
        name: "Pure Data",
        extensions: &["pd"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
