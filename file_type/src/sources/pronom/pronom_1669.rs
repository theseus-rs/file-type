use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const PRONOM_1669: FileType = FileType {
    file_format: &FileFormat {
        id: 1_669,
        source_type: SourceType::Pronom,
        name: "STL (Standard Tessellation Language) Binary",
        extensions: &["stl"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
