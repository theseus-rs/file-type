use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const PRONOM_864: FileType = FileType {
    file_format: &FileFormat {
        id: 864,
        source_type: SourceType::Pronom,
        name: "Steel Detailing Neutral Format",
        extensions: &["sdn"],
        media_types: &["text/plain"],
        signatures: &[],
        related_formats: &[],
    },
};
