use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const PRONOM_2254: FileType = FileType {
    file_format: &FileFormat {
        id: 2_254,
        source_type: SourceType::Pronom,
        name: "Minitab Project",
        extensions: &["mpj"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
