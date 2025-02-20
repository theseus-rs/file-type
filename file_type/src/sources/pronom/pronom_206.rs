use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const PRONOM_206: FileType = FileType {
    file_format: &FileFormat {
        id: 206,
        source_type: SourceType::Pronom,
        name: "Stats+ Data File",
        extensions: &[],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
