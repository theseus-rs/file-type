use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_1414196563: FileType = FileType {
    file_format: &FileFormat {
        id: 1_414_196_563,
        source_type: SourceType::Httpd,
        name: "zip",
        extensions: &["zip"],
        media_types: &["application/zip"],
        signatures: &[],
        related_formats: &[],
    },
};
