use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const PRONOM_2256: FileType = FileType {
    file_format: &FileFormat {
        id: 2_256,
        source_type: SourceType::Pronom,
        name: "Minitab Project",
        extensions: &["mpj"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
