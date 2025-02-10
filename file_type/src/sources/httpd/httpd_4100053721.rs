use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_4100053721: FileType = FileType {
    file_format: &FileFormat {
        id: 4_100_053_721,
        source_type: SourceType::Httpd,
        name: "adobe fxp",
        extensions: &["fxp", "fxpl"],
        media_types: &["application/vnd.adobe.fxp"],
        signatures: &[],
        related_formats: &[],
    },
};
