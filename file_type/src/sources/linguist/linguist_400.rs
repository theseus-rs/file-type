use crate::format::FileFormat;

pub(crate) const LINGUIST_400: FileFormat = FileFormat {
    id: 400,
    puid: "linguist/400",
    name: "XPages",
    extensions: &["xsp-config", "xsp.metadata"],
    media_types: &["text/xml"],
    internal_signatures: &[],
    related_formats: &[],
};
