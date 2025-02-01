use crate::format::FileFormat;

pub(crate) const LINGUIST_325: FileFormat = FileFormat {
    id: 325,
    puid: "linguist/325",
    name: "Rouge",
    extensions: &["rg"],
    media_types: &["text/x-clojure"],
    internal_signatures: &[],
    related_formats: &[],
};
