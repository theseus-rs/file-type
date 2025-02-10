use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const PRONOM_464: FileType = FileType {
    file_format: &FileFormat {
        id: 464,
        source_type: SourceType::Pronom,
        name: "AutoSketch Drawing",
        extensions: &["skf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
