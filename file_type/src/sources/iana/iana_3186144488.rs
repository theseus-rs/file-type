use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3186144488: FileFormat = FileFormat {
    id: 3_186_144_488,
    source_type: SourceType::Iana,
    name: "plain",
    extensions: &[],
    media_types: &["text/plain"],
    signatures: &[],
    related_formats: &[],
};
