use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_118465354: FileType = FileType {
    file_format: &FileFormat {
        id: 118_465_354,
        source_type: SourceType::Wikidata,
        name: "Capture One Session File",
        extensions: &["cos"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
