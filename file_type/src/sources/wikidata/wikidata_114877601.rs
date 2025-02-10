use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_114877601: FileType = FileType {
    file_format: &FileFormat {
        id: 114_877_601,
        source_type: SourceType::Wikidata,
        name: "Scrapbook Factory Deluxe Sticker file",
        extensions: &["sb"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
