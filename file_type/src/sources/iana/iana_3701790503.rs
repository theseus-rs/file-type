use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3701790503: FileFormat = FileFormat {
    id: 3_701_790_503,
    source_type: SourceType::Iana,
    name: "vnd.dynageo",
    extensions: &[],
    media_types: &["application/vnd.dynageo"],
    signatures: &[],
    related_formats: &[],
};
