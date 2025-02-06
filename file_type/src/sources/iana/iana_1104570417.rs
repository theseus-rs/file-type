use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1104570417: FileFormat = FileFormat {
    id: 1_104_570_417,
    source_type: SourceType::Iana,
    name: "disposition-notification",
    extensions: &[],
    media_types: &["message/disposition-notification"],
    signatures: &[],
    related_formats: &[],
};
