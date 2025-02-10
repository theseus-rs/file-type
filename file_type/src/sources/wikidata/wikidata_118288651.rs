use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_118288651: FileType = FileType {
    file_format: &FileFormat {
        id: 118_288_651,
        source_type: SourceType::Wikidata,
        name: "OnMark 2000 Project File",
        extensions: &["era"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
