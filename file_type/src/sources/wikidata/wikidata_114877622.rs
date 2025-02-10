use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_114877622: FileType = FileType {
    file_format: &FileFormat {
        id: 114_877_622,
        source_type: SourceType::Wikidata,
        name: "Scrapbook Factory Deluxe Family Tree file",
        extensions: &["sft"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
