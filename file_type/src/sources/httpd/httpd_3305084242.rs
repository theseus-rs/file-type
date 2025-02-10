use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_3305084242: FileType = FileType {
    file_format: &FileFormat {
        id: 3_305_084_242,
        source_type: SourceType::Httpd,
        name: "dssc xml",
        extensions: &["xdssc"],
        media_types: &["application/dssc+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
