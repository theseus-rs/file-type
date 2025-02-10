use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1096262999: FileType = FileType {
    file_format: &FileFormat {
        id: 1_096_262_999,
        source_type: SourceType::Iana,
        name: "avcs",
        extensions: &[],
        media_types: &["image/avcs"],
        signatures: &[],
        related_formats: &[],
    },
};
