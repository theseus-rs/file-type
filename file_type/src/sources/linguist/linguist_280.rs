use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_280: FileFormat = FileFormat {
    id: 280,
    source_type: SourceType::Linguist,
    name: "Parrot Internal Representation",
    extensions: &["pir"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
