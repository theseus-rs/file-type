use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_27925713: FileType = FileType {
    file_format: &FileFormat {
        id: 27_925_713,
        source_type: SourceType::Wikidata,
        name: "DTED Level 1 Gazetteer Primary file",
        extensions: &["gaz"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
