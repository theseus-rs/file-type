use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_111841144: FileType = FileType {
    file_format: &FileFormat {
        id: 111_841_144,
        source_type: SourceType::Wikidata,
        name: "JSON Lines",
        extensions: &["jsonl"],
        media_types: &["application/jsonl"],
        signatures: &[],
        related_formats: &[],
    },
};
