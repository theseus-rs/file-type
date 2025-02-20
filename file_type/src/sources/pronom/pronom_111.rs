use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const PRONOM_111: FileType = FileType {
    file_format: &FileFormat {
        id: 111,
        source_type: SourceType::Pronom,
        name: "AutoCAD Menu Resource File",
        extensions: &["mnr", "mnt"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
