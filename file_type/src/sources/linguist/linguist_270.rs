use crate::format::FileFormat;

pub(crate) const LINGUIST_270: FileFormat = FileFormat {
    id: 270,
    puid: "linguist/270",
    name: "Oz",
    extensions: &["oz"],
    media_types: &["text/x-oz"],
    internal_signatures: &[],
    related_formats: &[],
};
