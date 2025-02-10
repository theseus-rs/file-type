use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_27967114: FileType = FileType {
    file_format: &FileFormat {
        id: 27_967_114,
        source_type: SourceType::Wikidata,
        name: "Arkos Tracker",
        extensions: &["aks"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
