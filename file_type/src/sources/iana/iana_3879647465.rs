use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3879647465: FileFormat = FileFormat {
    id: 3_879_647_465,
    source_type: SourceType::Iana,
    name: "PCMA",
    extensions: &[],
    media_types: &["audio/PCMA"],
    signatures: &[],
    related_formats: &[],
};
