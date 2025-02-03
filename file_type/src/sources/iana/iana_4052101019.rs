use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_4052101019: FileFormat = FileFormat {
    id: 4_052_101_019,
    source_type: SourceType::Iana,
    name: "news-checkgroups",
    extensions: &[],
    media_types: &["application/news-checkgroups"],
    internal_signatures: &[],
    related_formats: &[],
};
