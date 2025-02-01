use crate::format::FileFormat;

pub(crate) const LINGUIST_402: FileFormat = FileFormat {
    id: 402,
    puid: "linguist/402",
    name: "XQuery",
    extensions: &["xq", "xql", "xqm", "xquery", "xqy"],
    media_types: &["application/xquery"],
    internal_signatures: &[],
    related_formats: &[],
};
