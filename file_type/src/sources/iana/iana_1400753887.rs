use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1400753887: FileFormat = FileFormat {
    id: 1_400_753_887,
    source_type: SourceType::Iana,
    name: "css",
    extensions: &[],
    media_types: &["text/css"],
    internal_signatures: &[],
    related_formats: &[],
};
