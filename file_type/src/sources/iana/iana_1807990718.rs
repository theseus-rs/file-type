use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1807990718: FileFormat = FileFormat {
    id: 1_807_990_718,
    source_type: SourceType::Iana,
    name: "mbms-register+xml",
    extensions: &[],
    media_types: &["application/mbms-register+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
