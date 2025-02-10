use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_125298268: FileType = FileType {
    file_format: &FileFormat {
        id: 125_298_268,
        source_type: SourceType::Wikidata,
        name: "Scribble",
        extensions: &["scrbl"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
