use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_719519: FileType = FileType {
    file_format: &FileFormat {
        id: 719_519,
        source_type: SourceType::Wikidata,
        name: "Forsyth–Edwards Notation",
        extensions: &["fen"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
