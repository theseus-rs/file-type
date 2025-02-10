use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_113486947: FileType = FileType {
    file_format: &FileFormat {
        id: 113_486_947,
        source_type: SourceType::Wikidata,
        name: "Persuasion Mac Document 3.0",
        extensions: &["pr3"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
