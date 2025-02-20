use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_126181123: FileType = FileType {
    file_format: &FileFormat {
        id: 126_181_123,
        source_type: SourceType::Wikidata,
        name: "Graphisoft Archicad Project 6-9",
        extensions: &["pla", "pln"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
