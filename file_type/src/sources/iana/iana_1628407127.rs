use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1628407127: FileFormat = FileFormat {
    id: 1_628_407_127,
    source_type: SourceType::Iana,
    name: "mathml-presentation+xml",
    extensions: &[],
    media_types: &["application/mathml-presentation+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
