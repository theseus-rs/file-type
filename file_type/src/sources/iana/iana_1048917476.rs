use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1048917476: FileType = FileType {
    file_format: &FileFormat {
        id: 1_048_917_476,
        source_type: SourceType::Iana,
        name: "xcap-diff+xml",
        extensions: &[],
        media_types: &["application/xcap-diff+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
