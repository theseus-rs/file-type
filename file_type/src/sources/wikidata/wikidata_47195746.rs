use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_47195746: FileType = FileType {
    file_format: &FileFormat {
        id: 47_195_746,
        source_type: SourceType::Wikidata,
        name: "AppleWorks Word Processor file format, version 5",
        extensions: &["cwk"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
