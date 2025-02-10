use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_938193433: FileType = FileType {
    file_format: &FileFormat {
        id: 938_193_433,
        source_type: SourceType::Linguist,
        name: "MiniZinc Data",
        extensions: &["dzn"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
