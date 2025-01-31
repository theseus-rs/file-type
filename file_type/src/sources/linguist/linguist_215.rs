use crate::format::FileFormat;

pub(crate) const LINGUIST_215: FileFormat = FileFormat {
    id: 215,
    puid: "linguist/215",
    name: "M4",
    extensions: &["m4", "mc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
