use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1685128235: FileFormat = FileFormat {
    id: 1_685_128_235,
    source_type: SourceType::Iana,
    name: "city+json",
    extensions: &[],
    media_types: &["application/city+json"],
    internal_signatures: &[],
    related_formats: &[],
};
