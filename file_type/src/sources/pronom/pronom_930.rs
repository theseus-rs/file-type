use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const PRONOM_930: FileType = FileType {
    file_format: &FileFormat {
        id: 930,
        source_type: SourceType::Pronom,
        name: "RealVideo Clip",
        extensions: &["rv"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
