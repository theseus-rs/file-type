use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const PRONOM_317: FileType = FileType {
    file_format: &FileFormat {
        id: 317,
        source_type: SourceType::Pronom,
        name: "ESRI MapInfo Data File",
        extensions: &["mid"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
