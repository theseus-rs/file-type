use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_117448593: FileType = FileType {
    file_format: &FileFormat {
        id: 117_448_593,
        source_type: SourceType::Wikidata,
        name: "FLExText Interlinear XML Format",
        extensions: &["flextext"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
