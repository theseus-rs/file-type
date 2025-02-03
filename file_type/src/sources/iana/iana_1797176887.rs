use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1797176887: FileFormat = FileFormat {
    id: 1_797_176_887,
    source_type: SourceType::Iana,
    name: "jpm",
    extensions: &[],
    media_types: &["image/jpm"],
    internal_signatures: &[],
    related_formats: &[],
};
