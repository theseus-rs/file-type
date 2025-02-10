use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const PRONOM_2580: FileType = FileType {
    file_format: &FileFormat {
        id: 2_580,
        source_type: SourceType::Pronom,
        name: "C/C++ Header File",
        extensions: &["h", "hpp", "hxx"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
