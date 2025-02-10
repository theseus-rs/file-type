use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_1035647: FileType = FileType {
    file_format: &FileFormat {
        id: 1_035_647,
        source_type: SourceType::Wikidata,
        name: "Card Verifiable Certificate",
        extensions: &["cv", "cvcert"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
