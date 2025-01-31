use crate::format::FileFormat;

pub(crate) const LINGUIST_50: FileFormat = FileFormat {
    id: 50,
    puid: "linguist/50",
    name: "CSS",
    extensions: &["css"],
    media_types: &["text/css"],
    internal_signatures: &[],
    related_formats: &[],
};
