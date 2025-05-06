use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_12018160: FileType = FileType {
    file_format: &FileFormat {
        id: 12_018_160,
        source_type: SourceType::Wikidata,
        name: "Geodatabase (Esri)",
        extensions: &["geodatabase"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
