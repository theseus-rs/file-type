use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3160782084: FileFormat = FileFormat {
    id: 3_160_782_084,
    source_type: SourceType::Iana,
    name: "rtf",
    extensions: &[],
    media_types: &["text/rtf"],
    internal_signatures: &[],
    related_formats: &[],
};
