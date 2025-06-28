use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_29945412: FileType = FileType {
    file_format: &FileFormat {
        id: 29_945_412,
        source_type: SourceType::Wikidata,
        name: "Rhino 3D Model, version 3",
        extensions: &["3dm"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
