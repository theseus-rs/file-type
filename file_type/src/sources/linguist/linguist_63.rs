use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_63: FileType = FileType {
    file_format: &FileFormat {
        id: 63,
        source_type: SourceType::Linguist,
        name: "CoffeeScript",
        extensions: &["_coffee", "cake", "cjsx", "coffee", "iced"],
        media_types: &["text/x-coffeescript"],
        signatures: &[],
        related_formats: &[],
    },
};
