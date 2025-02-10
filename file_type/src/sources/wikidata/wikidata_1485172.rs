use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_1485172: FileType = FileType {
    file_format: &FileFormat {
        id: 1_485_172,
        source_type: SourceType::Wikidata,
        name: "GENealogical inDEX",
        extensions: &["gendex.txt"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
