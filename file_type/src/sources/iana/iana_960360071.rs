use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_960360071: FileFormat = FileFormat {
    id: 960_360_071,
    source_type: SourceType::Iana,
    name: "vnd.hyper-item+json",
    extensions: &[],
    media_types: &["application/vnd.hyper-item+json"],
    internal_signatures: &[],
    related_formats: &[],
};
