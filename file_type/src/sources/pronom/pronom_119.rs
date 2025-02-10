use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const PRONOM_119: FileType = FileType {
    file_format: &FileFormat {
        id: 119,
        source_type: SourceType::Pronom,
        name: "AutoCAD Plot Configuration File",
        extensions: &["pc2"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
