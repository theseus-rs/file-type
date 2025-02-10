use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_29904540: FileType = FileType {
    file_format: &FileFormat {
        id: 29_904_540,
        source_type: SourceType::Wikidata,
        name: "Statistical Analysis System log file",
        extensions: &["log"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
