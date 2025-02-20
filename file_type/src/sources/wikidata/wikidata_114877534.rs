use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_114877534: FileType = FileType {
    file_format: &FileFormat {
        id: 114_877_534,
        source_type: SourceType::Wikidata,
        name: "Scrapbook Factory Deluxe Photo Card file",
        extensions: &["sph"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
