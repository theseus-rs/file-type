use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_113487065: FileType = FileType {
    file_format: &FileFormat {
        id: 113_487_065,
        source_type: SourceType::Wikidata,
        name: "Persuasion Windows Document 2.0",
        extensions: &["at2", "pr2"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
