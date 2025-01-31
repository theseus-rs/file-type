use crate::format::FileFormat;

pub(crate) const LINGUIST_176: FileFormat = FileFormat {
    id: 176,
    puid: "linguist/176",
    name: "JSONLD",
    extensions: &["jsonld"],
    media_types: &["application/json"],
    internal_signatures: &[],
    related_formats: &[],
};
