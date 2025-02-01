use crate::format::FileFormat;

pub(crate) const LINGUIST_319: FileFormat = FileFormat {
    id: 319,
    puid: "linguist/319",
    name: "Rebol",
    extensions: &["r", "r2", "r3", "reb", "rebol"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
