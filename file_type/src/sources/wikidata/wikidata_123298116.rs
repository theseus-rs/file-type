use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_123298116: FileType = FileType {
    file_format: &FileFormat {
        id: 123_298_116,
        source_type: SourceType::Wikidata,
        name: "To Do Archive",
        extensions: &["tda"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
