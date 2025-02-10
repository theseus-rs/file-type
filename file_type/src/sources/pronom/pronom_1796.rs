use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const PRONOM_1796: FileType = FileType {
    file_format: &FileFormat {
        id: 1_796,
        source_type: SourceType::Pronom,
        name: "SHA256 File",
        extensions: &["sha256"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
