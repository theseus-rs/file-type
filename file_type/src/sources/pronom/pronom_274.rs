use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const PRONOM_274: FileType = FileType {
    file_format: &FileFormat {
        id: 274,
        source_type: SourceType::Pronom,
        name: "PageMaker Time Stamp File",
        extensions: &["tym"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
