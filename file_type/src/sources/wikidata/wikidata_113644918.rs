use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_113644918: FileType = FileType {
    file_format: &FileFormat {
        id: 113_644_918,
        source_type: SourceType::Wikidata,
        name: "Intel SatisFAXtion",
        extensions: &["dcx"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
