use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_114888949: FileType = FileType {
    file_format: &FileFormat {
        id: 114_888_949,
        source_type: SourceType::Wikidata,
        name: "Scrapbook Factory Deluxe Photo Cube file",
        extensions: &["spq"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
