use crate::format::FileFormat;

pub(crate) const LINGUIST_25: FileFormat = FileFormat {
    id: 25,
    puid: "linguist/25",
    name: "Augeas",
    extensions: &["aug"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
