use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1037674256: FileFormat = FileFormat {
    id: 1_037_674_256,
    source_type: SourceType::Iana,
    name: "vnd.oma.poc.invocation-descriptor+xml",
    extensions: &[],
    media_types: &["application/vnd.oma.poc.invocation-descriptor+xml"],
    signatures: &[],
    related_formats: &[],
};
