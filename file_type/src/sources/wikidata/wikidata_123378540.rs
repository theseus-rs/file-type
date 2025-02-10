use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_123378540: FileType = FileType {
    file_format: &FileFormat {
        id: 123_378_540,
        source_type: SourceType::Wikidata,
        name: "Light library file",
        extensions: &["lgl"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
