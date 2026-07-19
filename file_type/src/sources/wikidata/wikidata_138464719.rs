use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_138464719: FileType = FileType {
    file_format: &FileFormat {
        id: 138_464_719,
        source_type: SourceType::Wikidata,
        name: "Rhino 3D Model, version 6",
        extensions: &["3dm"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
