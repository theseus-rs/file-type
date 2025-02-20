use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const PRONOM_194: FileType = FileType {
    file_format: &FileFormat {
        id: 194,
        source_type: SourceType::Pronom,
        name: "Audio Interchange File Format",
        extensions: &[],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
