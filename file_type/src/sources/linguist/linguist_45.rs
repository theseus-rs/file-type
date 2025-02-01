use crate::format::FileFormat;

pub(crate) const LINGUIST_45: FileFormat = FileFormat {
    id: 45,
    puid: "linguist/45",
    name: "C2hs Haskell",
    extensions: &["chs"],
    media_types: &["text/x-haskell"],
    internal_signatures: &[],
    related_formats: &[],
};
