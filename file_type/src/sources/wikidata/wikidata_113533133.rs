use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_113533133: FileType = FileType {
    file_format: &FileFormat {
        id: 113_533_133,
        source_type: SourceType::Wikidata,
        name: "LegalDocML Document",
        extensions: &["xml"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
