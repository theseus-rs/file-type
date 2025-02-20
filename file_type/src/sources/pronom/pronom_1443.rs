use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const PRONOM_1443: FileType = FileType {
    file_format: &FileFormat {
        id: 1_443,
        source_type: SourceType::Pronom,
        name: "Nullsoft Scriptable Install System",
        extensions: &["nsi"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
