use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const PRONOM_854: FileType = FileType {
    file_format: &FileFormat {
        id: 854,
        source_type: SourceType::Pronom,
        name: "AutoCAD Database File Locking Information",
        extensions: &["dwl"],
        media_types: &["application/octet-stream"],
        signatures: &[],
        related_formats: &[],
    },
};
