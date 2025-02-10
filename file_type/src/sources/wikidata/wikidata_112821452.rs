use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_112821452: FileType = FileType {
    file_format: &FileFormat {
        id: 112_821_452,
        source_type: SourceType::Wikidata,
        name: "Pro/ENGINEER rendering data file",
        extensions: &["slp"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
