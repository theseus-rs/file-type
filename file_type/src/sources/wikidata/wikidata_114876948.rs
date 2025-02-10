use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_114876948: FileType = FileType {
    file_format: &FileFormat {
        id: 114_876_948,
        source_type: SourceType::Wikidata,
        name: "Quicken Home Inventory file",
        extensions: &["idb"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
