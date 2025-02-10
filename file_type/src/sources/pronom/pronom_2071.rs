use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const PRONOM_2071: FileType = FileType {
    file_format: &FileFormat {
        id: 2_071,
        source_type: SourceType::Pronom,
        name: "ESRI Code Page File",
        extensions: &["cpg"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
