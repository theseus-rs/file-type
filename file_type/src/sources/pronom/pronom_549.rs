use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const PRONOM_549: FileType = FileType {
    file_format: &FileFormat {
        id: 549,
        source_type: SourceType::Pronom,
        name: "XYWrite Document",
        extensions: &["xy4"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
