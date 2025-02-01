use crate::format::FileFormat;

pub(crate) const LINGUIST_119: FileFormat = FileFormat {
    id: 119,
    puid: "linguist/119",
    name: "GAP",
    extensions: &["g", "gap", "gd", "gi", "tst"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
