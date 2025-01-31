use crate::format::FileFormat;

pub(crate) const LINGUIST_380: FileFormat = FileFormat {
    id: 380,
    puid: "linguist/380",
    name: "Unity3D Asset",
    extensions: &["anim", "asset", "mask", "mat", "meta", "prefab", "unity"],
    media_types: &["text/x-yaml"],
    internal_signatures: &[],
    related_formats: &[],
};
