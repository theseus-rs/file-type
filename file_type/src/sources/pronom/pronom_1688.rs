use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const PRONOM_1688: FileType = FileType {
    file_format: &FileFormat {
        id: 1_688,
        source_type: SourceType::Pronom,
        name: "AXD HTTP Handler File",
        extensions: &["axd"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
