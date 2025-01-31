use crate::format::FileFormat;

pub(crate) const DEFAULT_1: FileFormat = FileFormat {
    id: 1,
    puid: "default/1",
    name: "Binary",
    extensions: &[],
    media_types: &["application/octet-stream"],
    internal_signatures: &[],
    related_formats: &[],
};
