use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_62561275: FileType = FileType {
    file_format: &FileFormat {
        id: 62_561_275,
        source_type: SourceType::Wikidata,
        name: "Hangul Word Processor Document, version 5",
        extensions: &["hwp"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
