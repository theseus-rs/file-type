use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const PRONOM_524: FileType = FileType {
    file_format: &FileFormat {
        id: 524,
        source_type: SourceType::Pronom,
        name: "Silicon Graphics Graphics File",
        extensions: &[],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
