use crate::format::FileFormat;

pub(crate) const LINGUIST_78: FileFormat = FileFormat {
    id: 78,
    puid: "linguist/78",
    name: "Cycript",
    extensions: &["cy"],
    media_types: &["text/javascript"],
    internal_signatures: &[],
    related_formats: &[],
};
