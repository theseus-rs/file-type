use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const PRONOM_1317: FileType = FileType {
    file_format: &FileFormat {
        id: 1_317,
        source_type: SourceType::Pronom,
        name: "eRuby HTML document",
        extensions: &["rhtml", "rhtm"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
