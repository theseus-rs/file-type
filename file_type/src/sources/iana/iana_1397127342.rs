use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1397127342: FileFormat = FileFormat {
    id: 1_397_127_342,
    source_type: SourceType::Iana,
    name: "SGML",
    extensions: &[],
    media_types: &["application/SGML"],
    signatures: &[],
    related_formats: &[],
};
