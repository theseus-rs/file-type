use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1729556303: FileType = FileType {
    file_format: &FileFormat {
        id: 1_729_556_303,
        source_type: SourceType::Iana,
        name: "GSM-HR-08",
        extensions: &[],
        media_types: &["audio/GSM-HR-08"],
        signatures: &[],
        related_formats: &[],
    },
};
