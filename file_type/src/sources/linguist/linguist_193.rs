use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_193: FileType = FileType {
    file_format: &FileFormat {
        id: 193,
        source_type: SourceType::Linguist,
        name: "LSL",
        extensions: &["lsl", "lslp"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
