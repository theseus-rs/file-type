use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const PRONOM_2575: FileType = FileType {
    file_format: &FileFormat {
        id: 2_575,
        source_type: SourceType::Pronom,
        name: "Data File",
        extensions: &["dat"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
