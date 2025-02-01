use crate::format::FileFormat;

pub(crate) const LINGUIST_177: FileFormat = FileFormat {
    id: 177,
    puid: "linguist/177",
    name: "JSONiq",
    extensions: &["jq"],
    media_types: &["application/json"],
    internal_signatures: &[],
    related_formats: &[],
};
