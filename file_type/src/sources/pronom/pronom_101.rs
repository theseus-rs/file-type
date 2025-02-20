use crate::FileType;
use crate::format::{FileFormat, SourceType};

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
