use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_27823194: FileType = FileType {
    file_format: &FileFormat {
        id: 27_823_194,
        source_type: SourceType::Wikidata,
        name: "Binary Terrain, version 1.2",
        extensions: &["bt"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
