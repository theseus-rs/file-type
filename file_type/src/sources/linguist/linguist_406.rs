use crate::FileType;
use crate::format::{FileFormat, SourceType};

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
