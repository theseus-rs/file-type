use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_114888746: FileType = FileType {
    file_format: &FileFormat {
        id: 114_888_746,
        source_type: SourceType::Wikidata,
        name: "Scrapbook Factory Deluxe Envelope file",
        extensions: &["sev"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
