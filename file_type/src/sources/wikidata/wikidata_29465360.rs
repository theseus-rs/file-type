use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_29465360: FileType = FileType {
    file_format: &FileFormat {
        id: 29_465_360,
        source_type: SourceType::Wikidata,
        name: "VPM",
        extensions: &["vpm"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
