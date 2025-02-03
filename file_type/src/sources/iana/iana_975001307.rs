use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_975001307: FileFormat = FileFormat {
    id: 975_001_307,
    source_type: SourceType::Iana,
    name: "vnd.oma.poc.groups+xml",
    extensions: &[],
    media_types: &["application/vnd.oma.poc.groups+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
