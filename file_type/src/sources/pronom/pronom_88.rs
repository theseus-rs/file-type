use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const PRONOM_88: FileType = FileType {
    file_format: &FileFormat {
        id: 88,
        source_type: SourceType::Pronom,
        name: "AutoCAD Font Mapping Table",
        extensions: &["fmp"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
