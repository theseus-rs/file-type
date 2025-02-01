use crate::format::FileFormat;

pub(crate) const LINGUIST_193: FileFormat = FileFormat {
    id: 193,
    puid: "linguist/193",
    name: "LSL",
    extensions: &["lsl", "lslp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
