use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_100596946: FileType = FileType {
    file_format: &FileFormat {
        id: 100_596_946,
        source_type: SourceType::Wikidata,
        name: "Minitab Worksheet, version 14",
        extensions: &["mtw"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
