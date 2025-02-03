use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_503630353: FileFormat = FileFormat {
    id: 503_630_353,
    source_type: SourceType::Iana,
    name: "mp21",
    extensions: &[],
    media_types: &["application/mp21"],
    internal_signatures: &[],
    related_formats: &[],
};
