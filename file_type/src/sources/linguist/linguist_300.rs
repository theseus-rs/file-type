use crate::format::FileFormat;

pub(crate) const LINGUIST_300: FileFormat = FileFormat {
    id: 300,
    puid: "linguist/300",
    name: "Pure Data",
    extensions: &["pd"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
