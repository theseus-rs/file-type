use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2619251320: FileFormat = FileFormat {
    id: 2_619_251_320,
    source_type: SourceType::Iana,
    name: "gltf-binary",
    extensions: &[],
    media_types: &["model/gltf-binary"],
    internal_signatures: &[],
    related_formats: &[],
};
