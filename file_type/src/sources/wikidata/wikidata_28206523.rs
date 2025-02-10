use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28206523: FileType = FileType {
    file_format: &FileFormat {
        id: 28_206_523,
        source_type: SourceType::Wikidata,
        name: "LuraWave",
        extensions: &["lwf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
