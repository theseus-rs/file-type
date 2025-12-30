use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const PRONOM_3942: FileType = FileType {
    file_format: &FileFormat {
        id: 3_942,
        source_type: SourceType::Pronom,
        name: "XYZ Coordinate Data",
        extensions: &["xyz"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
