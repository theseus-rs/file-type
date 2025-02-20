use crate::FileType;
use crate::format::{FileFormat, SourceType};

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
