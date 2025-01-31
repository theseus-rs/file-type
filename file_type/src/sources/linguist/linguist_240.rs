use crate::format::FileFormat;

pub(crate) const LINGUIST_240: FileFormat = FileFormat {
    id: 240,
    puid: "linguist/240",
    name: "NCL",
    extensions: &["ncl"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
