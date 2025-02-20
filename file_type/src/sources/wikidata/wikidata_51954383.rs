use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_51954383: FileType = FileType {
    file_format: &FileFormat {
        id: 51_954_383,
        source_type: SourceType::Wikidata,
        name: "WordStar for MS-DOS Document, version 5.5",
        extensions: &["ws"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
