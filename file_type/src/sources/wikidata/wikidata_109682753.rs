use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_109682753: FileType = FileType {
    file_format: &FileFormat {
        id: 109_682_753,
        source_type: SourceType::Wikidata,
        name: "WinAce Archive",
        extensions: &["rar"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
