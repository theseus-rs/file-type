use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_182763210: FileFormat = FileFormat {
    id: 182_763_210,
    source_type: SourceType::Iana,
    name: "vnd.motorola.reflex",
    extensions: &[],
    media_types: &["text/vnd.motorola.reflex"],
    internal_signatures: &[],
    related_formats: &[],
};
