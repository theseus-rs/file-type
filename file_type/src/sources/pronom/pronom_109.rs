use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const PRONOM_109: FileType = FileType {
    file_format: &FileFormat {
        id: 109,
        source_type: SourceType::Pronom,
        name: "AutoCAD Compiled Menu",
        extensions: &["mnc"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
