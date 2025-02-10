use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_51917556: FileType = FileType {
    file_format: &FileFormat {
        id: 51_917_556,
        source_type: SourceType::Wikidata,
        name: "WordStar for Windows Document",
        extensions: &["ws", "wsd", "wsw"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
