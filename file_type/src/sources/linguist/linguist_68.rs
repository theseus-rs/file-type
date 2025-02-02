use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_68: FileFormat = FileFormat {
    id: 68,
    source_type: SourceType::Linguist,
    name: "Cool",
    extensions: &["cl"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
