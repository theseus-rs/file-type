use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3559334345: FileFormat = FileFormat {
    id: 3_559_334_345,
    source_type: SourceType::Iana,
    name: "news-transmission",
    extensions: &[],
    media_types: &["application/news-transmission"],
    internal_signatures: &[],
    related_formats: &[],
};
