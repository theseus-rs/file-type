use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_2407196801: FileType = FileType {
    file_format: &FileFormat {
        id: 2_407_196_801,
        source_type: SourceType::Httpd,
        name: "metalink xml",
        extensions: &["metalink"],
        media_types: &["application/metalink+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
