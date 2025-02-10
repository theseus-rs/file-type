use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const PRONOM_82: FileType = FileType {
    file_format: &FileFormat {
        id: 82,
        source_type: SourceType::Pronom,
        name: "AutoCAD Drawing Standards File",
        extensions: &["dws"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
