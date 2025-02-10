use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_116445963: FileType = FileType {
    file_format: &FileFormat {
        id: 116_445_963,
        source_type: SourceType::Wikidata,
        name: "CoffeeCup Web Video Player File",
        extensions: &["xml"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
