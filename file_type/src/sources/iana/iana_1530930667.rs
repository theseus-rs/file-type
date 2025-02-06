use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1530930667: FileFormat = FileFormat {
    id: 1_530_930_667,
    source_type: SourceType::Iana,
    name: "mbms-schedule+xml",
    extensions: &[],
    media_types: &["application/mbms-schedule+xml"],
    signatures: &[],
    related_formats: &[],
};
