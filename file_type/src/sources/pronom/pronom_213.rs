use crate::FileType;
use crate::format::{FileFormat, SourceType};

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
