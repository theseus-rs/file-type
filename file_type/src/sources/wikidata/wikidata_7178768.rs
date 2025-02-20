use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_7178768: FileType = FileType {
    file_format: &FileFormat {
        id: 7_178_768,
        source_type: SourceType::Wikidata,
        name: "Petri Net Markup Language",
        extensions: &["pnml"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
