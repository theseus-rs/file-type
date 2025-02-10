use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_113274736: FileType = FileType {
    file_format: &FileFormat {
        id: 113_274_736,
        source_type: SourceType::Wikidata,
        name: "The Print Shop Deluxe Online Greeting",
        extensions: &["pso"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
