use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_114877188: FileType = FileType {
    file_format: &FileFormat {
        id: 114_877_188,
        source_type: SourceType::Wikidata,
        name: "Money 1.0, 2.0, and 3.0 Backup File",
        extensions: &["bak"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
