use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const PRONOM_2252: FileType = FileType {
    file_format: &FileFormat {
        id: 2_252,
        source_type: SourceType::Pronom,
        name: "Minitab Project",
        extensions: &["mpj"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
