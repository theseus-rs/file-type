use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3859418788: FileFormat = FileFormat {
    id: 3_859_418_788,
    source_type: SourceType::Iana,
    name: "n-triples",
    extensions: &[],
    media_types: &["application/n-triples"],
    signatures: &[],
    related_formats: &[],
};
