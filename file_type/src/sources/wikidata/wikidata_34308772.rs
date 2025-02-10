use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_34308772: FileType = FileType {
    file_format: &FileFormat {
        id: 34_308_772,
        source_type: SourceType::Wikidata,
        name: "Scrivener document",
        extensions: &["scriv"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
