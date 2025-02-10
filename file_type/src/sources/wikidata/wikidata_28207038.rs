use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28207038: FileType = FileType {
    file_format: &FileFormat {
        id: 28_207_038,
        source_type: SourceType::Wikidata,
        name: "Photo Line Document",
        extensions: &["pld"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
