use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_117035683: FileType = FileType {
    file_format: &FileFormat {
        id: 117_035_683,
        source_type: SourceType::Wikidata,
        name: "FloorPlan file",
        extensions: &["bmf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
