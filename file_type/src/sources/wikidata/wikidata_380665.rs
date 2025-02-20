use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_380665: FileType = FileType {
    file_format: &FileFormat {
        id: 380_665,
        source_type: SourceType::Wikidata,
        name: "PLS",
        extensions: &["pls"],
        media_types: &["audio/x-scpls"],
        signatures: &[],
        related_formats: &[],
    },
};
