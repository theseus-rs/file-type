use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1213052401: FileFormat = FileFormat {
    id: 1_213_052_401,
    source_type: SourceType::Iana,
    name: "vnd.openxmlformats-officedocument.presentationml.comments+xml",
    extensions: &[],
    media_types: &["application/vnd.openxmlformats-officedocument.presentationml.comments+xml"],
    signatures: &[],
    related_formats: &[],
};
