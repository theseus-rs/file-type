use crate::format::FileFormat;

pub(crate) const LINGUIST_403: FileFormat = FileFormat {
    id: 403,
    puid: "linguist/403",
    name: "XS",
    extensions: &["xs"],
    media_types: &["text/x-csrc"],
    internal_signatures: &[],
    related_formats: &[],
};
