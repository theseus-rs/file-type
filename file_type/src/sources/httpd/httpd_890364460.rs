use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_890364460: FileType = FileType {
    file_format: &FileFormat {
        id: 890_364_460,
        source_type: SourceType::Httpd,
        name: "wais source",
        extensions: &["src"],
        media_types: &["application/x-wais-source"],
        signatures: &[],
        related_formats: &[],
    },
};
