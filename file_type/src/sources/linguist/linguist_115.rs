use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_115: FileFormat = FileFormat {
    id: 115,
    source_type: SourceType::Linguist,
    name: "FreeMarker",
    extensions: &["ftl"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
