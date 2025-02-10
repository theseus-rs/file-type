use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_284531423: FileType = FileType {
    file_format: &FileFormat {
        id: 284_531_423,
        source_type: SourceType::Linguist,
        name: "Jison",
        extensions: &["jison"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
