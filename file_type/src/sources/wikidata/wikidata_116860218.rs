use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_116860218: FileType = FileType {
    file_format: &FileFormat {
        id: 116_860_218,
        source_type: SourceType::Wikidata,
        name: "Forms Maker & Filler Forms file",
        extensions: &["dtp"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
