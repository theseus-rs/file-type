use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_52426038: FileType = FileType {
    file_format: &FileFormat {
        id: 52_426_038,
        source_type: SourceType::Wikidata,
        name: "WordStar for MS-DOS Document, version 3",
        extensions: &["ws", "ws3"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
