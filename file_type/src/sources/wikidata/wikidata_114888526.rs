use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_114888526: FileType = FileType {
    file_format: &FileFormat {
        id: 114_888_526,
        source_type: SourceType::Wikidata,
        name: "Scrapbook Factory Deluxe Craft file",
        extensions: &["sra"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
