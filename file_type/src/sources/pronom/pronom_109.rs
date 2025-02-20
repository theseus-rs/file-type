use crate::FileType;
use crate::format::{FileFormat, SourceType};

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
