use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2559453962: FileFormat = FileFormat {
    id: 2_559_453_962,
    source_type: SourceType::Iana,
    name: "calendar",
    extensions: &[],
    media_types: &["text/calendar"],
    signatures: &[],
    related_formats: &[],
};
