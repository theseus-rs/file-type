use crate::format::FileFormat;

pub(crate) const LINGUIST_126: FileFormat = FileFormat {
    id: 126,
    puid: "linguist/126",
    name: "Genshi",
    extensions: &["kid"],
    media_types: &["text/xml"],
    internal_signatures: &[],
    related_formats: &[],
};
