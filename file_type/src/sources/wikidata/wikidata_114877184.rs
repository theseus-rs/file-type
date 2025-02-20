use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_114877184: FileType = FileType {
    file_format: &FileFormat {
        id: 114_877_184,
        source_type: SourceType::Wikidata,
        name: "Money 95, 97, and 98 Backup File",
        extensions: &["mny"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
