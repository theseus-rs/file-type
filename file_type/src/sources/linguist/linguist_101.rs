use crate::format::FileFormat;

pub(crate) const LINGUIST_101: FileFormat = FileFormat {
    id: 101,
    puid: "linguist/101",
    name: "Elm",
    extensions: &["elm"],
    media_types: &["text/x-elm"],
    internal_signatures: &[],
    related_formats: &[],
};
