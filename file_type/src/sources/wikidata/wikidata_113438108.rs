use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_113438108: FileType = FileType {
    file_format: &FileFormat {
        id: 113_438_108,
        source_type: SourceType::Wikidata,
        name: "EndNote Library X - X9",
        extensions: &["enl"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
