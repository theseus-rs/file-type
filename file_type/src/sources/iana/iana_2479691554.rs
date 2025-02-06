use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2479691554: FileFormat = FileFormat {
    id: 2_479_691_554,
    source_type: SourceType::Iana,
    name: "spirits-event+xml",
    extensions: &[],
    media_types: &["application/spirits-event+xml"],
    signatures: &[],
    related_formats: &[],
};
