use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_3246778510: FileType = FileType {
    file_format: &FileFormat {
        id: 3_246_778_510,
        source_type: SourceType::Iana,
        name: "ccmp+xml",
        extensions: &[],
        media_types: &["application/ccmp+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
