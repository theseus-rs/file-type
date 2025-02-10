use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_3807693: FileType = FileType {
    file_format: &FileFormat {
        id: 3_807_693,
        source_type: SourceType::Wikidata,
        name: "ASCII tab",
        extensions: &["btab", "tab", "txt"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
