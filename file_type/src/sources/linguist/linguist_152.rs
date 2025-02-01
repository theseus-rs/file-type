use crate::format::FileFormat;

pub(crate) const LINGUIST_152: FileFormat = FileFormat {
    id: 152,
    puid: "linguist/152",
    name: "HTTP",
    extensions: &["http"],
    media_types: &["message/http"],
    internal_signatures: &[],
    related_formats: &[],
};
