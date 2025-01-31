use crate::format::FileFormat;

pub(crate) const LINGUIST_212: FileFormat = FileFormat {
    id: 212,
    puid: "linguist/212",
    name: "LoomScript",
    extensions: &["ls"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
