use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const PRONOM_16: FileType = FileType {
    file_format: &FileFormat {
        id: 16,
        source_type: SourceType::Pronom,
        name: "FoxPro Database",
        extensions: &["dbf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
