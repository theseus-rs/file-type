use crate::format::FileFormat;

pub(crate) const LINGUIST_190: FileFormat = FileFormat {
    id: 190,
    puid: "linguist/190",
    name: "LFE",
    extensions: &["lfe"],
    media_types: &["text/x-common-lisp"],
    internal_signatures: &[],
    related_formats: &[],
};
