use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_114877371: FileType = FileType {
    file_format: &FileFormat {
        id: 114_877_371,
        source_type: SourceType::Wikidata,
        name: "Scrapbook Factory Deluxe Scrapbook file",
        extensions: &["sbk"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
