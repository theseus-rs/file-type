use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_4144641364: FileType = FileType {
    file_format: &FileFormat {
        id: 4_144_641_364,
        source_type: SourceType::Httpd,
        name: "xliff xml",
        extensions: &["xlf"],
        media_types: &["application/x-xliff+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
