use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2898430558: FileFormat = FileFormat {
    id: 2_898_430_558,
    source_type: SourceType::Iana,
    name: "jais",
    extensions: &[],
    media_types: &["image/jais"],
    internal_signatures: &[],
    related_formats: &[],
};
