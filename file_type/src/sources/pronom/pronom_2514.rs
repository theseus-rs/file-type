use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const PRONOM_2514: FileType = FileType {
    file_format: &FileFormat {
        id: 2_514,
        source_type: SourceType::Pronom,
        name: "MATLAB Script File",
        extensions: &["m"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
