use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2189656763: FileFormat = FileFormat {
    id: 2_189_656_763,
    source_type: SourceType::Iana,
    name: "vnd.collection+json",
    extensions: &[],
    media_types: &["application/vnd.collection+json"],
    signatures: &[],
    related_formats: &[],
};
