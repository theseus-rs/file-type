use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_604682788: FileFormat = FileFormat {
    id: 604_682_788,
    source_type: SourceType::Iana,
    name: "gltf+json",
    extensions: &[],
    media_types: &["model/gltf+json"],
    signatures: &[],
    related_formats: &[],
};
