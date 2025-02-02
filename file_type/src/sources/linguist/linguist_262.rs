use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_262: FileFormat = FileFormat {
    id: 262,
    source_type: SourceType::Linguist,
    name: "Opal",
    extensions: &["opal"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
