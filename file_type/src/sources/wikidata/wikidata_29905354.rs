use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_29905354: FileType = FileType {
    file_format: &FileFormat {
        id: 29_905_354,
        source_type: SourceType::Wikidata,
        name: "Self-contained Information Retention Format",
        extensions: &["json", "xml"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
