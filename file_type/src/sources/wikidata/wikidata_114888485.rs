use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_114888485: FileType = FileType {
    file_format: &FileFormat {
        id: 114_888_485,
        source_type: SourceType::Wikidata,
        name: "Scrapbook Factory Deluxe Web Album file",
        extensions: &["swa"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
