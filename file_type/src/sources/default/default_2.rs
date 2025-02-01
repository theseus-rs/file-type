use crate::format::FileFormat;

pub(crate) const DEFAULT_2: FileFormat = FileFormat {
    id: 2,
    puid: "default/2",
    name: "Text",
    extensions: &[],
    media_types: &["text/plain"],
    internal_signatures: &[],
    related_formats: &[],
};
