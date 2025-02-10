use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_27967380: FileType = FileType {
    file_format: &FileFormat {
        id: 27_967_380,
        source_type: SourceType::Wikidata,
        name: "EVT",
        extensions: &["evt"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
