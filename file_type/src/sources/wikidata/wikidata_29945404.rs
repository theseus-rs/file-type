use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_29945404: FileType = FileType {
    file_format: &FileFormat {
        id: 29_945_404,
        source_type: SourceType::Wikidata,
        name: "Rhino 3D Model, version 1",
        extensions: &["3dm"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
