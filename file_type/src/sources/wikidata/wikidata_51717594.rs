use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_51717594: FileType = FileType {
    file_format: &FileFormat {
        id: 51_717_594,
        source_type: SourceType::Wikidata,
        name: "Pocket Word Template",
        extensions: &["pwt"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
