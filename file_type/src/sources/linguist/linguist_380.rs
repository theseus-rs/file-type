use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_380: FileFormat = FileFormat {
    id: 380,
    source_type: SourceType::Linguist,
    name: "Unity3D Asset",
    extensions: &["anim", "asset", "mask", "mat", "meta", "prefab", "unity"],
    media_types: &["text/x-yaml"],
    internal_signatures: &[],
    related_formats: &[],
};
