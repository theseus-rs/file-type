use crate::format::FileFormat;

pub(crate) const LINGUIST_17: FileFormat = FileFormat {
    id: 17,
    puid: "linguist/17",
    name: "Apex",
    extensions: &["apex", "cls", "trigger"],
    media_types: &["text/x-java"],
    internal_signatures: &[],
    related_formats: &[],
};
