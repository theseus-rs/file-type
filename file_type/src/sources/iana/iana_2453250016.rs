use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2453250016: FileFormat = FileFormat {
    id: 2_453_250_016,
    source_type: SourceType::Iana,
    name: "kpml-request+xml",
    extensions: &[],
    media_types: &["application/kpml-request+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
