use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_27823193: FileType = FileType {
    file_format: &FileFormat {
        id: 27_823_193,
        source_type: SourceType::Wikidata,
        name: "Binary Terrain, version 1.1",
        extensions: &["bt"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
