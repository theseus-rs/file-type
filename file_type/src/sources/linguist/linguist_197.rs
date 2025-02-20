use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_197: FileType = FileType {
    file_format: &FileFormat {
        id: 197,
        source_type: SourceType::Linguist,
        name: "Lean",
        extensions: &["hlean", "lean"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
