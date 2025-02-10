use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_51954568: FileType = FileType {
    file_format: &FileFormat {
        id: 51_954_568,
        source_type: SourceType::Wikidata,
        name: "WordStar for Windows Document, version 2",
        extensions: &["ws", "wsw"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
