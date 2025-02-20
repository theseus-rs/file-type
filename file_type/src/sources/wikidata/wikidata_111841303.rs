use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_111841303: FileType = FileType {
    file_format: &FileFormat {
        id: 111_841_303,
        source_type: SourceType::Wikidata,
        name: "line-delimited JSON",
        extensions: &["ldjson"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
