use crate::format::FileFormat;

pub(crate) const LINGUIST_337: FileFormat = FileFormat {
    id: 337,
    puid: "linguist/337",
    name: "SVG",
    extensions: &["svg"],
    media_types: &["text/xml"],
    internal_signatures: &[],
    related_formats: &[],
};
