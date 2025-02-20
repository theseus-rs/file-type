use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const PRONOM_793: FileType = FileType {
    file_format: &FileFormat {
        id: 793,
        source_type: SourceType::Pronom,
        name: "JPEG-LS",
        extensions: &["jls"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
