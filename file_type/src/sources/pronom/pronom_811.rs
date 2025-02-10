use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const PRONOM_811: FileType = FileType {
    file_format: &FileFormat {
        id: 811,
        source_type: SourceType::Pronom,
        name: "Deluxe Paint bitmap",
        extensions: &["lbm"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
