use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_122412029: FileType = FileType {
    file_format: &FileFormat {
        id: 122_412_029,
        source_type: SourceType::Wikidata,
        name: "FileMaker Runtime File",
        extensions: &["syo"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
