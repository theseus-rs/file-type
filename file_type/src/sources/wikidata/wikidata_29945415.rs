use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_29945415: FileType = FileType {
    file_format: &FileFormat {
        id: 29_945_415,
        source_type: SourceType::Wikidata,
        name: "Rhino 3D Model, version 4",
        extensions: &["3dm"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
