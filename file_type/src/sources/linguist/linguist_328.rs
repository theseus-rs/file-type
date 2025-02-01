use crate::format::FileFormat;

pub(crate) const LINGUIST_328: FileFormat = FileFormat {
    id: 328,
    puid: "linguist/328",
    name: "SAS",
    extensions: &["sas"],
    media_types: &["text/x-sas"],
    internal_signatures: &[],
    related_formats: &[],
};
