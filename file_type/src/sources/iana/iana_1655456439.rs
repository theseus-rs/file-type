use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1655456439: FileFormat = FileFormat {
    id: 1_655_456_439,
    source_type: SourceType::Iana,
    name: "news (OBSOLETED by [RFC5537])",
    extensions: &[],
    media_types: &["message/news"],
    internal_signatures: &[],
    related_formats: &[],
};
