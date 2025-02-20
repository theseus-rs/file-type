use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const PRONOM_103: FileType = FileType {
    file_format: &FileFormat {
        id: 103,
        source_type: SourceType::Pronom,
        name: "Log File",
        extensions: &["log"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
