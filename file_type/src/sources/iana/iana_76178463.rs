use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_76178463: FileFormat = FileFormat {
    id: 76_178_463,
    source_type: SourceType::Iana,
    name: "vnd.amadeus+json",
    extensions: &[],
    media_types: &["application/vnd.amadeus+json"],
    internal_signatures: &[],
    related_formats: &[],
};
