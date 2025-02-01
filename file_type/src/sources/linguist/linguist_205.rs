use crate::format::FileFormat;

pub(crate) const LINGUIST_205: FileFormat = FileFormat {
    id: 205,
    puid: "linguist/205",
    name: "Literate Agda",
    extensions: &["lagda"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
