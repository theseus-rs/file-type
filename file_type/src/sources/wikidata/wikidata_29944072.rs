use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_29944072: FileType = FileType {
    file_format: &FileFormat {
        id: 29_944_072,
        source_type: SourceType::Wikidata,
        name: "Simple Voxels",
        extensions: &["svx"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
