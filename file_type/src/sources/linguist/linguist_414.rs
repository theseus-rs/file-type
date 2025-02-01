use crate::format::FileFormat;

pub(crate) const LINGUIST_414: FileFormat = FileFormat {
    id: 414,
    puid: "linguist/414",
    name: "edn",
    extensions: &["edn"],
    media_types: &["text/x-clojure"],
    internal_signatures: &[],
    related_formats: &[],
};
