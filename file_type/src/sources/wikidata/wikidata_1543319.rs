use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_1543319: FileType = FileType {
    file_format: &FileFormat {
        id: 1_543_319,
        source_type: SourceType::Wikidata,
        name: "Graph Modelling Language",
        extensions: &["gml"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
