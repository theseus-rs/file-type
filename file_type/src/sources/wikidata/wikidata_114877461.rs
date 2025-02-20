use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_114877461: FileType = FileType {
    file_format: &FileFormat {
        id: 114_877_461,
        source_type: SourceType::Wikidata,
        name: "Scrapbook Factory Deluxe Mini Album file",
        extensions: &["sma"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
