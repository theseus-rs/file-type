use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const PRONOM_1519: FileType = FileType {
    file_format: &FileFormat {
        id: 1_519,
        source_type: SourceType::Pronom,
        name: "MBOX",
        extensions: &["mbox"],
        media_types: &["application/mbox"],
        signatures: &[],
        related_formats: &[],
    },
};
