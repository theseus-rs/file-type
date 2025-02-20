use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_73019618: FileType = FileType {
    file_format: &FileFormat {
        id: 73_019_618,
        source_type: SourceType::Wikidata,
        name: "WordStar for MS-DOS Document, version 7.0",
        extensions: &["ws7"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
