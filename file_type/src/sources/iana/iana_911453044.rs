use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_911453044: FileFormat = FileFormat {
    id: 911_453_044,
    source_type: SourceType::Iana,
    name: "vnd.biopax.rdf+xml",
    extensions: &[],
    media_types: &["application/vnd.biopax.rdf+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
