use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_60264007: FileType = FileType {
    file_format: &FileFormat {
        id: 60_264_007,
        source_type: SourceType::Iana,
        name: "xcap-att+xml",
        extensions: &[],
        media_types: &["application/xcap-att+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
