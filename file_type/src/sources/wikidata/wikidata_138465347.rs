use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_138465347: FileType = FileType {
    file_format: &FileFormat {
        id: 138_465_347,
        source_type: SourceType::Wikidata,
        name: "Rhino 3D Model, version 8",
        extensions: &["3dm"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
