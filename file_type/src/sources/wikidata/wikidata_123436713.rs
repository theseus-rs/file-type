use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_123436713: FileType = FileType {
    file_format: &FileFormat {
        id: 123_436_713,
        source_type: SourceType::Wikidata,
        name: "Single-Molecule Dataset file",
        extensions: &["smd"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
