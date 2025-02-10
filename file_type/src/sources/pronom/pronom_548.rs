use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const PRONOM_548: FileType = FileType {
    file_format: &FileFormat {
        id: 548,
        source_type: SourceType::Pronom,
        name: "XYWrite Document",
        extensions: &["xyp"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
