use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_21040799: FileType = FileType {
    file_format: &FileFormat {
        id: 21_040_799,
        source_type: SourceType::Wikidata,
        name: "MadTracker 2 format",
        extensions: &["mt2"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
