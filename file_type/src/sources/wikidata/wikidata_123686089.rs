use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_123686089: FileType = FileType {
    file_format: &FileFormat {
        id: 123_686_089,
        source_type: SourceType::Wikidata,
        name: "Adobe InDesign Document, version CC 2024",
        extensions: &["ind", "indd", "indt"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
