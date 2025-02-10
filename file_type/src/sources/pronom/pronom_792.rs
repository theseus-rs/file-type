use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const PRONOM_792: FileType = FileType {
    file_format: &FileFormat {
        id: 792,
        source_type: SourceType::Pronom,
        name: "JTIP (JPEG Tiled Image Pyramid)",
        extensions: &[],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
