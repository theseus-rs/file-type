use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const PRONOM_321: FileType = FileType {
    file_format: &FileFormat {
        id: 321,
        source_type: SourceType::Pronom,
        name: "Intergraph Raster Image",
        extensions: &["ing"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
