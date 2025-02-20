use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_116446090: FileType = FileType {
    file_format: &FileFormat {
        id: 116_446_090,
        source_type: SourceType::Wikidata,
        name: "CoffeeCup Web JukeBox File",
        extensions: &["xml"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
