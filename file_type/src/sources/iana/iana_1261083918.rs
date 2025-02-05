use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1261083918: FileFormat = FileFormat {
    id: 1_261_083_918,
    source_type: SourceType::Iana,
    name: "vnd.HandHeld-Entertainment+xml",
    extensions: &[],
    media_types: &["application/vnd.HandHeld-Entertainment+xml"],
    signatures: &[],
    related_formats: &[],
};
