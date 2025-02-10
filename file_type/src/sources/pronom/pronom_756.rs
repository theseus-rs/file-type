use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const PRONOM_756: FileType = FileType {
    file_format: &FileFormat {
        id: 756,
        source_type: SourceType::Pronom,
        name: "StarOffice Draw",
        extensions: &[],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
