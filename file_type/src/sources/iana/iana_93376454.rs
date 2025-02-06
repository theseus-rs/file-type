use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_93376454: FileFormat = FileFormat {
    id: 93_376_454,
    source_type: SourceType::Iana,
    name: "private-token-request",
    extensions: &[],
    media_types: &["application/private-token-request"],
    signatures: &[],
    related_formats: &[],
};
