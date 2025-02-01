use crate::format::FileFormat;

pub(crate) const LINGUIST_219: FileFormat = FileFormat {
    id: 219,
    puid: "linguist/219",
    name: "MUF",
    extensions: &["m", "muf"],
    media_types: &["text/x-forth"],
    internal_signatures: &[],
    related_formats: &[],
};
