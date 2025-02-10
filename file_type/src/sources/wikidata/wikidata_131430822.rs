use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_131430822: FileType = FileType {
    file_format: &FileFormat {
        id: 131_430_822,
        source_type: SourceType::Wikidata,
        name: "XQuery Source File",
        extensions: &["xq", "xql", "xqm", "xquery", "xqy"],
        media_types: &["application/xquery", "text/xquery"],
        signatures: &[],
        related_formats: &[],
    },
};
