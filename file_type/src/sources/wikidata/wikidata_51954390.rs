use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_51954390: FileType = FileType {
    file_format: &FileFormat {
        id: 51_954_390,
        source_type: SourceType::Wikidata,
        name: "WordStar for MS-DOS Document, version 6",
        extensions: &["ws", "ws6"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
