use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const PRONOM_404: FileType = FileType {
    file_format: &FileFormat {
        id: 404,
        source_type: SourceType::Pronom,
        name: "dBASE Database",
        extensions: &["dbf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
