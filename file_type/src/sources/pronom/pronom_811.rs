use crate::FileType;
use crate::format::{FileFormat, SourceType};

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
