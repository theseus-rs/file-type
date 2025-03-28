use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const PRONOM_939: FileType = FileType {
    file_format: &FileFormat {
        id: 939,
        source_type: SourceType::Pronom,
        name: "ScanIt Document",
        extensions: &["sid"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
