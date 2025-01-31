use crate::format::FileFormat;

pub(crate) const LINGUIST_51: FileFormat = FileFormat {
    id: 51,
    puid: "linguist/51",
    name: "CSV",
    extensions: &["csv"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
