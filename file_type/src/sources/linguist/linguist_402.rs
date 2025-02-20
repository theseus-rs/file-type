use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_402: FileType = FileType {
    file_format: &FileFormat {
        id: 402,
        source_type: SourceType::Linguist,
        name: "XQuery",
        extensions: &["xq", "xql", "xqm", "xquery", "xqy"],
        media_types: &["application/xquery"],
        signatures: &[],
        related_formats: &[],
    },
};
