use crate::format::FileFormat;

pub(crate) const LINGUIST_373: FileFormat = FileFormat {
    id: 373,
    puid: "linguist/373",
    name: "Textile",
    extensions: &["textile"],
    media_types: &["text/x-textile"],
    internal_signatures: &[],
    related_formats: &[],
};
