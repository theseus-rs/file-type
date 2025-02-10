use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_59630618: FileType = FileType {
    file_format: &FileFormat {
        id: 59_630_618,
        source_type: SourceType::Wikidata,
        name: "Scriptware Script Format",
        extensions: &["sw3"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
