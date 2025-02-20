use crate::FileType;
use crate::format::{FileFormat, SourceType};

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
