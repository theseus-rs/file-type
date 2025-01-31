use crate::format::FileFormat;

pub(crate) const LINGUIST_132: FileFormat = FileFormat {
    id: 132,
    puid: "linguist/132",
    name: "Go",
    extensions: &["go"],
    media_types: &["text/x-go"],
    internal_signatures: &[],
    related_formats: &[],
};
