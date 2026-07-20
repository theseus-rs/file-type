use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_137654239: FileType = FileType {
    file_format: &FileFormat {
        id: 137_654_239,
        source_type: SourceType::Wikidata,
        name: "Token-Oriented Object Notation",
        extensions: &["toon"],
        media_types: &["text/toon"],
        signatures: &[],
        related_formats: &[],
    },
};
