use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_129: FileFormat = FileFormat {
    id: 129,
    source_type: SourceType::Linguist,
    name: "Gettext Catalog",
    extensions: &["po", "pot"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
