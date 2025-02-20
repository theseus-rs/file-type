use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_114877374: FileType = FileType {
    file_format: &FileFormat {
        id: 114_877_374,
        source_type: SourceType::Wikidata,
        name: "Scrapbook Factory Deluxe Journal file",
        extensions: &["sjd"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
