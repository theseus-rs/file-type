use crate::format::FileFormat;

pub(crate) const LINGUIST_64: FileFormat = FileFormat {
    id: 64,
    puid: "linguist/64",
    name: "ColdFusion",
    extensions: &["cfm", "cfml"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
