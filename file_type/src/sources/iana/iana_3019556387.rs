use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_3019556387: FileType = FileType {
    file_format: &FileFormat {
        id: 3_019_556_387,
        source_type: SourceType::Iana,
        name: "thraud+xml",
        extensions: &[],
        media_types: &["application/thraud+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
