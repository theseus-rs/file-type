use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const PRONOM_468: FileType = FileType {
    file_format: &FileFormat {
        id: 468,
        source_type: SourceType::Pronom,
        name: "CorelCHART Document",
        extensions: &["cch"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
