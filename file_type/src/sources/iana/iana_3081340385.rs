use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3081340385: FileFormat = FileFormat {
    id: 3_081_340_385,
    source_type: SourceType::Iana,
    name: "vnd.xacml+json",
    extensions: &[],
    media_types: &["application/vnd.xacml+json"],
    signatures: &[],
    related_formats: &[],
};
