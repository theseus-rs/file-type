use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_2271124225: FileType = FileType {
    file_format: &FileFormat {
        id: 2_271_124_225,
        source_type: SourceType::Httpd,
        name: "rar compressed",
        extensions: &["rar"],
        media_types: &["application/x-rar-compressed"],
        signatures: &[],
        related_formats: &[],
    },
};
