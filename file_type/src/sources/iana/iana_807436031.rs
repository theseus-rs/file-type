use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_807436031: FileFormat = FileFormat {
    id: 807_436_031,
    source_type: SourceType::Iana,
    name: "held+xml",
    extensions: &[],
    media_types: &["application/held+xml"],
    signatures: &[],
    related_formats: &[],
};
