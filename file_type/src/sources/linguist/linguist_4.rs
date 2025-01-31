use crate::format::FileFormat;

pub(crate) const LINGUIST_4: FileFormat = FileFormat {
    id: 4,
    puid: "linguist/4",
    name: "ANTLR",
    extensions: &["g4"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
