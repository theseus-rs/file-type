use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_206: FileType = FileType {
    file_format: &FileFormat {
        id: 206,
        source_type: SourceType::Linguist,
        name: "Literate CoffeeScript",
        extensions: &["coffee.md", "litcoffee"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
