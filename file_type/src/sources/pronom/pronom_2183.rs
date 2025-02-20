use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const PRONOM_2183: FileType = FileType {
    file_format: &FileFormat {
        id: 2_183,
        source_type: SourceType::Pronom,
        name: "Debug File",
        extensions: &["dbg"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
