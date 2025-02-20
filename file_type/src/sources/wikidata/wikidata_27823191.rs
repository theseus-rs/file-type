use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_27823191: FileType = FileType {
    file_format: &FileFormat {
        id: 27_823_191,
        source_type: SourceType::Wikidata,
        name: "Binary Terrain, version 1.0",
        extensions: &["bt"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
