use crate::format::FileFormat;

pub(crate) const LINGUIST_32: FileFormat = FileFormat {
    id: 32,
    puid: "linguist/32",
    name: "BitBake",
    extensions: &["bb", "bbappend", "bbclass", "inc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
