use crate::format::FileFormat;

pub(crate) const LINGUIST_1: FileFormat = FileFormat {
    id: 1,
    puid: "linguist/1",
    name: "ABAP",
    extensions: &["abap"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
