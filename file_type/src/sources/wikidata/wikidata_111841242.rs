use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_111841242: FileType = FileType {
    file_format: &FileFormat {
        id: 111_841_242,
        source_type: SourceType::Wikidata,
        name: "NDJSON",
        extensions: &["ndjson"],
        media_types: &["application/x-ndjson"],
        signatures: &[],
        related_formats: &[],
    },
};
