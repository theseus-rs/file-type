use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3893484041: FileFormat = FileFormat {
    id: 3_893_484_041,
    source_type: SourceType::Iana,
    name: "ld+json",
    extensions: &[],
    media_types: &["application/ld+json"],
    internal_signatures: &[],
    related_formats: &[],
};
