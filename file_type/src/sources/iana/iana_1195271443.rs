use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1195271443: FileType = FileType {
    file_format: &FileFormat {
        id: 1_195_271_443,
        source_type: SourceType::Iana,
        name: "vnd.ms-modi",
        extensions: &[],
        media_types: &["image/vnd.ms-modi"],
        signatures: &[],
        related_formats: &[],
    },
};
