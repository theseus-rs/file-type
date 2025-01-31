use crate::format::FileFormat;

pub(crate) const LINGUIST_20: FileFormat = FileFormat {
    id: 20,
    puid: "linguist/20",
    name: "Arc",
    extensions: &["arc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
