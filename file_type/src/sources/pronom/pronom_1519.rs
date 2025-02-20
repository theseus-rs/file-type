use crate::FileType;
use crate::format::{FileFormat, SourceType};

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
