use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_85413178: FileType = FileType {
    file_format: &FileFormat {
        id: 85_413_178,
        source_type: SourceType::Wikidata,
        name: "PTGui Project File 10",
        extensions: &["pts"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
