use crate::format::FileFormat;

pub(crate) const LINGUIST_106: FileFormat = FileFormat {
    id: 106,
    puid: "linguist/106",
    name: "FLUX",
    extensions: &["flux", "fx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
