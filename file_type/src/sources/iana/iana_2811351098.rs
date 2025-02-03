use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2811351098: FileFormat = FileFormat {
    id: 2_811_351_098,
    source_type: SourceType::Iana,
    name: "calendar+json",
    extensions: &[],
    media_types: &["application/calendar+json"],
    internal_signatures: &[],
    related_formats: &[],
};
