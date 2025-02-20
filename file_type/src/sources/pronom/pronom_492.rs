use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const PRONOM_492: FileType = FileType {
    file_format: &FileFormat {
        id: 492,
        source_type: SourceType::Pronom,
        name: "Interleaf Document",
        extensions: &["doc"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
