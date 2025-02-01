use crate::format::FileFormat;

pub(crate) const LINGUIST_165: FileFormat = FileFormat {
    id: 165,
    puid: "linguist/165",
    name: "Idris",
    extensions: &["idr", "lidr"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
