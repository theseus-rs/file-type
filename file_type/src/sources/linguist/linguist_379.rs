use crate::format::FileFormat;

pub(crate) const LINGUIST_379: FileFormat = FileFormat {
    id: 379,
    puid: "linguist/379",
    name: "Unified Parallel C",
    extensions: &["upc"],
    media_types: &["text/x-csrc"],
    internal_signatures: &[],
    related_formats: &[],
};
