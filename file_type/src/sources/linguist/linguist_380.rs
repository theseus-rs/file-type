use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_380: FileType = FileType {
    file_format: &FileFormat {
        id: 380,
        source_type: SourceType::Linguist,
        name: "Unity3D Asset",
        extensions: &["anim", "asset", "mask", "mat", "meta", "prefab", "unity"],
        media_types: &["text/x-yaml"],
        signatures: &[],
        related_formats: &[],
    },
};
