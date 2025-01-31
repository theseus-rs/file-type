use crate::format::FileFormat;

pub(crate) const LINGUIST_361: FileFormat = FileFormat {
    id: 361,
    puid: "linguist/361",
    name: "SuperCollider",
    extensions: &["sc", "scd"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
