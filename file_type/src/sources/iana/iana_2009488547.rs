use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2009488547: FileFormat = FileFormat {
    id: 2_009_488_547,
    source_type: SourceType::Iana,
    name: "sparql-query",
    extensions: &[],
    media_types: &["application/sparql-query"],
    internal_signatures: &[],
    related_formats: &[],
};
