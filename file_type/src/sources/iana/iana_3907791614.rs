use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3907791614: FileFormat = FileFormat {
    id: 3_907_791_614,
    source_type: SourceType::Iana,
    name: "vnd.cyclonedx+xml",
    extensions: &[],
    media_types: &["application/vnd.cyclonedx+xml"],
    signatures: &[],
    related_formats: &[],
};
