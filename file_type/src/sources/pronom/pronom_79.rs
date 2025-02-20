use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const PRONOM_79: FileType = FileType {
    file_format: &FileFormat {
        id: 79,
        source_type: SourceType::Pronom,
        name: "Visual Basic Macro",
        extensions: &["dvb"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
