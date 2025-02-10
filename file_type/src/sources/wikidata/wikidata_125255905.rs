use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_125255905: FileType = FileType {
    file_format: &FileFormat {
        id: 125_255_905,
        source_type: SourceType::Wikidata,
        name: "Simulation Result File",
        extensions: &["mat"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
