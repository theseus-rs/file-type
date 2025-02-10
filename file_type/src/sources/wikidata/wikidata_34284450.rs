use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_34284450: FileType = FileType {
    file_format: &FileFormat {
        id: 34_284_450,
        source_type: SourceType::Wikidata,
        name: "Pawn script",
        extensions: &["p"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
