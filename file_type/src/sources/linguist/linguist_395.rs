use crate::format::FileFormat;

pub(crate) const LINGUIST_395: FileFormat = FileFormat {
    id: 395,
    puid: "linguist/395",
    name: "WebIDL",
    extensions: &["webidl"],
    media_types: &["text/x-webidl"],
    internal_signatures: &[],
    related_formats: &[],
};
