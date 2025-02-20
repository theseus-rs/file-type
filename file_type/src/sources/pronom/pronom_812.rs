use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const PRONOM_812: FileType = FileType {
    file_format: &FileFormat {
        id: 812,
        source_type: SourceType::Pronom,
        name: "Generic Library File",
        extensions: &["lib"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
