use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_4118131962: FileFormat = FileFormat {
    id: 4_118_131_962,
    source_type: SourceType::Iana,
    name: "vnd.oma.poc.final-report+xml",
    extensions: &[],
    media_types: &["application/vnd.oma.poc.final-report+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
