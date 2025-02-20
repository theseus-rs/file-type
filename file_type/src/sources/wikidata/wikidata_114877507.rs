use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_114877507: FileType = FileType {
    file_format: &FileFormat {
        id: 114_877_507,
        source_type: SourceType::Wikidata,
        name: "Scrapbook Factory Deluxe Photo Project file",
        extensions: &["spp"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
