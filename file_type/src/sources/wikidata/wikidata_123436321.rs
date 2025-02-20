use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_123436321: FileType = FileType {
    file_format: &FileFormat {
        id: 123_436_321,
        source_type: SourceType::Wikidata,
        name: "DARC-F1 Query file",
        extensions: &["f1q"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
