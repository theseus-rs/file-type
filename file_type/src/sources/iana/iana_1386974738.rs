use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1386974738: FileFormat = FileFormat {
    id: 1_386_974_738,
    source_type: SourceType::Iana,
    name: "calendar+xml",
    extensions: &[],
    media_types: &["application/calendar+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
