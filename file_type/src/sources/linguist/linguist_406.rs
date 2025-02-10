use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_406: FileType = FileType {
    file_format: &FileFormat {
        id: 406,
        source_type: SourceType::Linguist,
        name: "Xtend",
        extensions: &["xtend"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
