use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_113438957: FileType = FileType {
    file_format: &FileFormat {
        id: 113_438_957,
        source_type: SourceType::Wikidata,
        name: "EndNote Library 20",
        extensions: &["enl"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
