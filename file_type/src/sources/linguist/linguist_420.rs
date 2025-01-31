use crate::format::FileFormat;

pub(crate) const LINGUIST_420: FileFormat = FileFormat {
    id: 420,
    puid: "linguist/420",
    name: "wisp",
    extensions: &["wisp"],
    media_types: &["text/x-clojure"],
    internal_signatures: &[],
    related_formats: &[],
};
