use crate::format::FileFormat;

pub(crate) const LINGUIST_91: FileFormat = FileFormat {
    id: 91,
    puid: "linguist/91",
    name: "Dylan",
    extensions: &["dyl", "dylan", "intr", "lid"],
    media_types: &["text/x-dylan"],
    internal_signatures: &[],
    related_formats: &[],
};
