use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1251956507: FileFormat = FileFormat {
    id: 1_251_956_507,
    source_type: SourceType::Iana,
    name: "3gppHalForms+json",
    extensions: &[],
    media_types: &["application/3gppHalForms+json"],
    internal_signatures: &[],
    related_formats: &[],
};
