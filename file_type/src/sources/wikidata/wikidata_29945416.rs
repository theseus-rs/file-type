use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_29945416: FileType = FileType {
    file_format: &FileFormat {
        id: 29_945_416,
        source_type: SourceType::Wikidata,
        name: "Rhino 3D Model, version 5",
        extensions: &["3dm"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
