use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_47197294: FileType = FileType {
    file_format: &FileFormat {
        id: 47_197_294,
        source_type: SourceType::Wikidata,
        name: "AppleWorks Word Processor file format version 6",
        extensions: &["cwk"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
