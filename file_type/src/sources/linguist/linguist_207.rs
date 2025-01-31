use crate::format::FileFormat;

pub(crate) const LINGUIST_207: FileFormat = FileFormat {
    id: 207,
    puid: "linguist/207",
    name: "Literate Haskell",
    extensions: &["lhs"],
    media_types: &["text/x-literate-haskell"],
    internal_signatures: &[],
    related_formats: &[],
};
