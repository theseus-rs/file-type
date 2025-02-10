use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const PRONOM_101: FileType = FileType {
    file_format: &FileFormat {
        id: 101,
        source_type: SourceType::Pronom,
        name: "AutoCAD Linetype Definition File",
        extensions: &["lin"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
