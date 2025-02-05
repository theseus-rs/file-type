use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_206: FileFormat = FileFormat {
    id: 206,
    source_type: SourceType::Linguist,
    name: "Literate CoffeeScript",
    extensions: &["coffee.md", "litcoffee"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
