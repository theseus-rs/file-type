use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_29945408: FileType = FileType {
    file_format: &FileFormat {
        id: 29_945_408,
        source_type: SourceType::Wikidata,
        name: "Rhino 3D Model, version 2",
        extensions: &["3dm"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
