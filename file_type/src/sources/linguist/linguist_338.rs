use crate::format::FileFormat;

pub(crate) const LINGUIST_338: FileFormat = FileFormat {
    id: 338,
    puid: "linguist/338",
    name: "Sage",
    extensions: &["sage", "sagews"],
    media_types: &["text/x-python"],
    internal_signatures: &[],
    related_formats: &[],
};
