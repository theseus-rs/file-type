use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_604682788: FileType = FileType {
    file_format: &FileFormat {
        id: 604_682_788,
        source_type: SourceType::Iana,
        name: "gltf+json",
        extensions: &[],
        media_types: &["model/gltf+json"],
        signatures: &[],
        related_formats: &[],
    },
};
