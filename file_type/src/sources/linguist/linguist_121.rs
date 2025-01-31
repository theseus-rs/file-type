use crate::format::FileFormat;

pub(crate) const LINGUIST_121: FileFormat = FileFormat {
    id: 121,
    puid: "linguist/121",
    name: "GCC Machine Description",
    extensions: &["md"],
    media_types: &["text/x-common-lisp"],
    internal_signatures: &[],
    related_formats: &[],
};
