use crate::format::FileFormat;

pub(crate) const LINGUIST_125: FileFormat = FileFormat {
    id: 125,
    puid: "linguist/125",
    name: "Game Maker Language",
    extensions: &["gml"],
    media_types: &["text/x-c++src"],
    internal_signatures: &[],
    related_formats: &[],
};
