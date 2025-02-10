use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_3280650409: FileType = FileType {
    file_format: &FileFormat {
        id: 3_280_650_409,
        source_type: SourceType::Httpd,
        name: "scvp cv response",
        extensions: &["scs"],
        media_types: &["application/scvp-cv-response"],
        signatures: &[],
        related_formats: &[],
    },
};
