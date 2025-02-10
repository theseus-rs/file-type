use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const PRONOM_2452: FileType = FileType {
    file_format: &FileFormat {
        id: 2_452,
        source_type: SourceType::Pronom,
        name: "ESRI Colour File Format",
        extensions: &["clr"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
