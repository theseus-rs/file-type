use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_62485511: FileType = FileType {
    file_format: &FileFormat {
        id: 62_485_511,
        source_type: SourceType::Wikidata,
        name: "Navisworks Document, version 2011",
        extensions: &["nwc", "nwd"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
