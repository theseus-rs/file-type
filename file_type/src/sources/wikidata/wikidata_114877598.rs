use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_114877598: FileType = FileType {
    file_format: &FileFormat {
        id: 114_877_598,
        source_type: SourceType::Wikidata,
        name: "Scrapbook Factory Deluxe Trading Card file",
        extensions: &["stc"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
