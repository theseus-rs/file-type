use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_2619251320: FileType = FileType {
    file_format: &FileFormat {
        id: 2_619_251_320,
        source_type: SourceType::Iana,
        name: "gltf-binary",
        extensions: &[],
        media_types: &["model/gltf-binary"],
        signatures: &[],
        related_formats: &[],
    },
};
