use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_29000618: FileType = FileType {
    file_format: &FileFormat {
        id: 29_000_618,
        source_type: SourceType::Wikidata,
        name: "Hiew Colour Markers",
        extensions: &["cmarkers"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
