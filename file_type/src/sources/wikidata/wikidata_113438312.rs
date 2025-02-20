use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_113438312: FileType = FileType {
    file_format: &FileFormat {
        id: 113_438_312,
        source_type: SourceType::Wikidata,
        name: "EndNote Compressed Library X - X9",
        extensions: &["enlx"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
