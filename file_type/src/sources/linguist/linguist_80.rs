use crate::format::FileFormat;

pub(crate) const LINGUIST_80: FileFormat = FileFormat {
    id: 80,
    puid: "linguist/80",
    name: "D",
    extensions: &["d", "di"],
    media_types: &["text/x-d"],
    internal_signatures: &[],
    related_formats: &[],
};
