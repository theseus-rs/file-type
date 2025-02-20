use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const PRONOM_503: FileType = FileType {
    file_format: &FileFormat {
        id: 503,
        source_type: SourceType::Pronom,
        name: "Lotus Notes File",
        extensions: &["box"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
