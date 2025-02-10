use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_114889200: FileType = FileType {
    file_format: &FileFormat {
        id: 114_889_200,
        source_type: SourceType::Wikidata,
        name: "Scrapbook Factory Deluxe Puzzle file",
        extensions: &["spz"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
