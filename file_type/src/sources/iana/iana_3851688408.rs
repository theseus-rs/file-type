use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3851688408: FileFormat = FileFormat {
    id: 3_851_688_408,
    source_type: SourceType::Iana,
    name: "x400-bp",
    extensions: &[],
    media_types: &["application/x400-bp"],
    internal_signatures: &[],
    related_formats: &[],
};
