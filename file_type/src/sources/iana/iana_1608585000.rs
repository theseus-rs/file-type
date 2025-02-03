use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1608585000: FileFormat = FileFormat {
    id: 1_608_585_000,
    source_type: SourceType::Iana,
    name: "vnd.valve.source.compiled-map",
    extensions: &[],
    media_types: &["model/vnd.valve.source.compiled-map"],
    internal_signatures: &[],
    related_formats: &[],
};
