use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3433762130: FileFormat = FileFormat {
    id: 3_433_762_130,
    source_type: SourceType::Iana,
    name: "rfc+xml",
    extensions: &[],
    media_types: &["application/rfc+xml"],
    signatures: &[],
    related_formats: &[],
};
