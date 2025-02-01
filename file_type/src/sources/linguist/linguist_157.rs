use crate::format::FileFormat;

pub(crate) const LINGUIST_157: FileFormat = FileFormat {
    id: 157,
    puid: "linguist/157",
    name: "Haskell",
    extensions: &["hs", "hs-boot", "hsc"],
    media_types: &["text/x-haskell"],
    internal_signatures: &[],
    related_formats: &[],
};
