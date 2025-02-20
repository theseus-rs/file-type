use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_113487211: FileType = FileType {
    file_format: &FileFormat {
        id: 113_487_211,
        source_type: SourceType::Wikidata,
        name: "Persuasion Windows Document 3 - 4",
        extensions: &["at3", "at4", "pn4", "pr3"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
