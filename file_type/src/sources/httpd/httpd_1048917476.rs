use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_1048917476: FileType = FileType {
    file_format: &FileFormat {
        id: 1_048_917_476,
        source_type: SourceType::Httpd,
        name: "xcap diff xml",
        extensions: &["xdf"],
        media_types: &["application/xcap-diff+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
