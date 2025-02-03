use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_155: FileFormat = FileFormat {
    id: 155,
    source_type: SourceType::Linguist,
    name: "Handlebars",
    extensions: &["handlebars", "hbs"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
