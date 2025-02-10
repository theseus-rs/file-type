use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_27996239: FileType = FileType {
    file_format: &FileFormat {
        id: 27_996_239,
        source_type: SourceType::Wikidata,
        name: "Faster than Light saved game",
        extensions: &["sav"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
