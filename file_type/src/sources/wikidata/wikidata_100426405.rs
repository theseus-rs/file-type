use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_100426405: FileType = FileType {
    file_format: &FileFormat {
        id: 100_426_405,
        source_type: SourceType::Wikidata,
        name: "Minitab Worksheet, version 13",
        extensions: &["mtw"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
