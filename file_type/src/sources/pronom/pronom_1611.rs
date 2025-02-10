use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const PRONOM_1611: FileType = FileType {
    file_format: &FileFormat {
        id: 1_611,
        source_type: SourceType::Pronom,
        name: "StarOffice Draw",
        extensions: &["sdd"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
