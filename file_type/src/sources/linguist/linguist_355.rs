use crate::format::FileFormat;

pub(crate) const LINGUIST_355: FileFormat = FileFormat {
    id: 355,
    puid: "linguist/355",
    name: "Squirrel",
    extensions: &["nut"],
    media_types: &["text/x-c++src"],
    internal_signatures: &[],
    related_formats: &[],
};
