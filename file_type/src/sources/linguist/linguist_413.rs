use crate::format::FileFormat;

pub(crate) const LINGUIST_413: FileFormat = FileFormat {
    id: 413,
    puid: "linguist/413",
    name: "eC",
    extensions: &["ec", "eh"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
