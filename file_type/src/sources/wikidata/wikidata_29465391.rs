use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_29465391: FileType = FileType {
    file_format: &FileFormat {
        id: 29_465_391,
        source_type: SourceType::Wikidata,
        name: "unified diff",
        extensions: &["diff", "patch"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
