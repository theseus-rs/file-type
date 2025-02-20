use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const PRONOM_238: FileType = FileType {
    file_format: &FileFormat {
        id: 238,
        source_type: SourceType::Pronom,
        name: "PICS Animation",
        extensions: &["pcs"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
