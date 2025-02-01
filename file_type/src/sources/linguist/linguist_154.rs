use crate::format::FileFormat;

pub(crate) const LINGUIST_154: FileFormat = FileFormat {
    id: 154,
    puid: "linguist/154",
    name: "Haml",
    extensions: &["haml", "haml.deface"],
    media_types: &["text/x-haml"],
    internal_signatures: &[],
    related_formats: &[],
};
