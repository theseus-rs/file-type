use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_49620191: FileType = FileType {
    file_format: &FileFormat {
        id: 49_620_191,
        source_type: SourceType::Wikidata,
        name: "Revit Project",
        extensions: &["rvt"],
        media_types: &["application/octet-stream"],
        signatures: &[],
        related_formats: &[],
    },
};
