use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const PRONOM_2000: FileType = FileType {
    file_format: &FileFormat {
        id: 2_000,
        source_type: SourceType::Pronom,
        name: "Adobe SWC Package",
        extensions: &["swc"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
