use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1368694713: FileFormat = FileFormat {
    id: 1_368_694_713,
    source_type: SourceType::Iana,
    name: "msword",
    extensions: &[],
    media_types: &["application/msword"],
    internal_signatures: &[],
    related_formats: &[],
};
