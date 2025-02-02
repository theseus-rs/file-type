use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_63: FileFormat = FileFormat {
    id: 63,
    source_type: SourceType::Linguist,
    name: "CoffeeScript",
    extensions: &["_coffee", "cake", "cjsx", "coffee", "iced"],
    media_types: &["text/x-coffeescript"],
    internal_signatures: &[],
    related_formats: &[],
};
