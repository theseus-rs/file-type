use crate::format::FileFormat;

pub(crate) const LINGUIST_318: FileFormat = FileFormat {
    id: 318,
    puid: "linguist/318",
    name: "Raw token data",
    extensions: &["raw"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
