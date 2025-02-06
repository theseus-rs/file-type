use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1721268295: FileFormat = FileFormat {
    id: 1_721_268_295,
    source_type: SourceType::Iana,
    name: "metalink4+xml",
    extensions: &[],
    media_types: &["application/metalink4+xml"],
    signatures: &[],
    related_formats: &[],
};
