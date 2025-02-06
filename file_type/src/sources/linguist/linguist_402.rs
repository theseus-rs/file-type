use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_402: FileFormat = FileFormat {
    id: 402,
    source_type: SourceType::Linguist,
    name: "XQuery",
    extensions: &["xq", "xql", "xqm", "xquery", "xqy"],
    media_types: &["application/xquery"],
    signatures: &[],
    related_formats: &[],
};
