use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_113401722: FileType = FileType {
    file_format: &FileFormat {
        id: 113_401_722,
        source_type: SourceType::Wikidata,
        name: "Linux/i386 Binary Executable File ZMAGIC",
        extensions: &["o", "so"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
