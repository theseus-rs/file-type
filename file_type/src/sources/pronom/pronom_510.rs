use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const PRONOM_510: FileType = FileType {
    file_format: &FileFormat {
        id: 510,
        source_type: SourceType::Pronom,
        name: "Microstation CAD Drawing",
        extensions: &["dgn"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
