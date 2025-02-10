use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_650915217: FileType = FileType {
    file_format: &FileFormat {
        id: 650_915_217,
        source_type: SourceType::Iana,
        name: "UEMCLIP",
        extensions: &[],
        media_types: &["audio/UEMCLIP"],
        signatures: &[],
        related_formats: &[],
    },
};
