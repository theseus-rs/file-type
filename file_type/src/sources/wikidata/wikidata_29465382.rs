use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_29465382: FileType = FileType {
    file_format: &FileFormat {
        id: 29_465_382,
        source_type: SourceType::Wikidata,
        name: "UltraEdit Project User Interface",
        extensions: &["pui"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
