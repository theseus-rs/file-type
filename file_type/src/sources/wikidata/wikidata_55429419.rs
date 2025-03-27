use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_55429419: FileType = FileType {
    file_format: &FileFormat {
        id: 55_429_419,
        source_type: SourceType::Wikidata,
        name: "Python bytecode, version 3.7",
        extensions: &["pyc"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
