use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1392872326: FileFormat = FileFormat {
    id: 1_392_872_326,
    source_type: SourceType::Iana,
    name: "route-apd+xml",
    extensions: &[],
    media_types: &["application/route-apd+xml"],
    signatures: &[],
    related_formats: &[],
};
