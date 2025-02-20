use crate::FileType;
use crate::format::{FileFormat, SourceType};

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
