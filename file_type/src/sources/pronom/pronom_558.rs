use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const PRONOM_558: FileType = FileType {
    file_format: &FileFormat {
        id: 558,
        source_type: SourceType::Pronom,
        name: "dBASE for Windows database",
        extensions: &["dbf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
