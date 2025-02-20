use crate::FileType;
use crate::format::{FileFormat, SourceType};

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
