use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const PRONOM_1466: FileType = FileType {
    file_format: &FileFormat {
        id: 1_466,
        source_type: SourceType::Pronom,
        name: "Photoshop Curve File",
        extensions: &["acv", "atf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
