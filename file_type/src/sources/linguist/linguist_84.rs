use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_84: FileFormat = FileFormat {
    id: 84,
    source_type: SourceType::Linguist,
    name: "DNS Zone",
    extensions: &["arpa", "zone"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
