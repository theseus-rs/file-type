use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_79239177: FileType = FileType {
    file_format: &FileFormat {
        id: 79_239_177,
        source_type: SourceType::Wikidata,
        name: "The Bat! Address Book",
        extensions: &["abd"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
