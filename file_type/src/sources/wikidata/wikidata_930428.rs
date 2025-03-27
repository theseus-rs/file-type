use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_930428: FileType = FileType {
    file_format: &FileFormat {
        id: 930_428,
        source_type: SourceType::Wikidata,
        name: "X3D",
        extensions: &["x3d"],
        media_types: &["model/x3d+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
