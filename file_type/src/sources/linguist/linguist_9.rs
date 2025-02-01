use crate::format::FileFormat;

pub(crate) const LINGUIST_9: FileFormat = FileFormat {
    id: 9,
    puid: "linguist/9",
    name: "ATS",
    extensions: &["dats", "hats", "sats"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
