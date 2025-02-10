use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const PRONOM_2666: FileType = FileType {
    file_format: &FileFormat {
        id: 2_666,
        source_type: SourceType::Pronom,
        name: "Adobe Color Swatch",
        extensions: &["aco"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
