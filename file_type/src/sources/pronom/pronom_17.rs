use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const PRONOM_17: FileType = FileType {
    file_format: &FileFormat {
        id: 17,
        source_type: SourceType::Pronom,
        name: "FoxPro Database",
        extensions: &["dbf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
