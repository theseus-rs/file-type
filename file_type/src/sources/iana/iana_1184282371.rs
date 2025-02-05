use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1184282371: FileFormat = FileFormat {
    id: 1_184_282_371,
    source_type: SourceType::Iana,
    name: "ccxml+xml",
    extensions: &[],
    media_types: &["application/ccxml+xml"],
    signatures: &[],
    related_formats: &[],
};
