use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_4042481: FileType = FileType {
    file_format: &FileFormat {
        id: 4_042_481,
        source_type: SourceType::Wikidata,
        name: "LOGML",
        extensions: &["xml"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
