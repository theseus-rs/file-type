use crate::format::FileFormat;

pub(crate) const LINGUIST_12: FileFormat = FileFormat {
    id: 12,
    puid: "linguist/12",
    name: "Agda",
    extensions: &["agda"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
