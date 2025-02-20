use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_915515483: FileType = FileType {
    file_format: &FileFormat {
        id: 915_515_483,
        source_type: SourceType::Iana,
        name: "html",
        extensions: &[],
        media_types: &["text/html"],
        signatures: &[],
        related_formats: &[],
    },
};
