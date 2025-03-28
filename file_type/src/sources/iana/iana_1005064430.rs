use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1005064430: FileType = FileType {
    file_format: &FileFormat {
        id: 1_005_064_430,
        source_type: SourceType::Iana,
        name: "gltf-buffer",
        extensions: &[],
        media_types: &["application/gltf-buffer"],
        signatures: &[],
        related_formats: &[],
    },
};
