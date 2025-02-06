use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1993893760: FileFormat = FileFormat {
    id: 1_993_893_760,
    source_type: SourceType::Iana,
    name: "vnd.cyclonedx+json",
    extensions: &[],
    media_types: &["application/vnd.cyclonedx+json"],
    signatures: &[],
    related_formats: &[],
};
