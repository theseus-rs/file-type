use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_100596960: FileType = FileType {
    file_format: &FileFormat {
        id: 100_596_960,
        source_type: SourceType::Wikidata,
        name: "Minitab Worksheet, version 15+",
        extensions: &["mtw"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
