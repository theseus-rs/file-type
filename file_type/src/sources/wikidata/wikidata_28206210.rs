use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28206210: FileType = FileType {
    file_format: &FileFormat {
        id: 28_206_210,
        source_type: SourceType::Wikidata,
        name: "Graph Saurus SR7/SR8/SRS",
        extensions: &["sr7", "sr8", "srs"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
