use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const PRONOM_237: FileType = FileType {
    file_format: &FileFormat {
        id: 237,
        source_type: SourceType::Pronom,
        name: "Kodak PhotoCD Image",
        extensions: &[],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
