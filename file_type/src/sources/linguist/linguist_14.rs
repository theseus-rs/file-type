use crate::format::FileFormat;

pub(crate) const LINGUIST_14: FileFormat = FileFormat {
    id: 14,
    puid: "linguist/14",
    name: "Alpine Abuild",
    extensions: &[],
    media_types: &["text/x-sh"],
    internal_signatures: &[],
    related_formats: &[],
};
