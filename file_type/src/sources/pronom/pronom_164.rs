use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const PRONOM_164: FileType = FileType {
    file_format: &FileFormat {
        id: 164,
        source_type: SourceType::Pronom,
        name: "AutoCAD External Database Configuration File",
        extensions: &["udl"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
