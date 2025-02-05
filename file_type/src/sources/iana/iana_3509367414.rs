use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3509367414: FileFormat = FileFormat {
    id: 3_509_367_414,
    source_type: SourceType::Iana,
    name: "mods+xml",
    extensions: &[],
    media_types: &["application/mods+xml"],
    signatures: &[],
    related_formats: &[],
};
