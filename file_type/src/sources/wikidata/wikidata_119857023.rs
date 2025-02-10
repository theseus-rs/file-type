use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_119857023: FileType = FileType {
    file_format: &FileFormat {
        id: 119_857_023,
        source_type: SourceType::Wikidata,
        name: "Map Template",
        extensions: &["stt"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
