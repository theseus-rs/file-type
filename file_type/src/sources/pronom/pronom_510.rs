use crate::FileType;
use crate::format::{FileFormat, SourceType};

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
