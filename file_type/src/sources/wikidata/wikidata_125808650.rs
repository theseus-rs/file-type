use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_125808650: FileType = FileType {
    file_format: &FileFormat {
        id: 125_808_650,
        source_type: SourceType::Wikidata,
        name: "Mnemosyne 2.0 file",
        extensions: &["db"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
