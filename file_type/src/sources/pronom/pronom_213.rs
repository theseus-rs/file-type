use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const PRONOM_213: FileType = FileType {
    file_format: &FileFormat {
        id: 213,
        source_type: SourceType::Pronom,
        name: "Micrografx Designer",
        extensions: &["dsf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
