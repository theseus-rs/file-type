use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_29904505: FileType = FileType {
    file_format: &FileFormat {
        id: 29_904_505,
        source_type: SourceType::Wikidata,
        name: "S7z",
        extensions: &["s7z"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
