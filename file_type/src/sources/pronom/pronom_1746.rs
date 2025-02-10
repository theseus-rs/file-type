use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const PRONOM_1746: FileType = FileType {
    file_format: &FileFormat {
        id: 1_746,
        source_type: SourceType::Pronom,
        name: "Back Up File",
        extensions: &["bak"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
