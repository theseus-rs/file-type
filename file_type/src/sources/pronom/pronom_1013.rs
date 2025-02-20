use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const PRONOM_1013: FileType = FileType {
    file_format: &FileFormat {
        id: 1_013,
        source_type: SourceType::Pronom,
        name: "SPSS Output File (spv)",
        extensions: &["spv"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
