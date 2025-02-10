use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_27861488: FileType = FileType {
    file_format: &FileFormat {
        id: 27_861_488,
        source_type: SourceType::Wikidata,
        name: "Renoise Song, version 21",
        extensions: &["xrns"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
