use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_128693921: FileType = FileType {
    file_format: &FileFormat {
        id: 128_693_921,
        source_type: SourceType::Wikidata,
        name: "BQN file",
        extensions: &["bqn"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
