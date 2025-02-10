use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_115241368: FileType = FileType {
    file_format: &FileFormat {
        id: 115_241_368,
        source_type: SourceType::Wikidata,
        name: "3D Builder Project",
        extensions: &["b3d"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
