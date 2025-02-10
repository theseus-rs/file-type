use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_113274636: FileType = FileType {
    file_format: &FileFormat {
        id: 113_274_636,
        source_type: SourceType::Wikidata,
        name: "The Print Shop Deluxe Post-It Note",
        extensions: &["ppi"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
