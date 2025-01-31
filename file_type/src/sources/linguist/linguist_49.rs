use crate::format::FileFormat;

pub(crate) const LINGUIST_49: FileFormat = FileFormat {
    id: 49,
    puid: "linguist/49",
    name: "COLLADA",
    extensions: &["dae"],
    media_types: &["text/xml"],
    internal_signatures: &[],
    related_formats: &[],
};
