use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_83159681: FileType = FileType {
    file_format: &FileFormat {
        id: 83_159_681,
        source_type: SourceType::Wikidata,
        name: "RWL",
        extensions: &["rwl"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
