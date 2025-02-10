use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_51802172: FileType = FileType {
    file_format: &FileFormat {
        id: 51_802_172,
        source_type: SourceType::Wikidata,
        name: "Speller Custom Dictionary",
        extensions: &["dic"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
