use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_3430275081: FileType = FileType {
    file_format: &FileFormat {
        id: 3_430_275_081,
        source_type: SourceType::Httpd,
        name: "ustar",
        extensions: &["ustar"],
        media_types: &["application/x-ustar"],
        signatures: &[],
        related_formats: &[],
    },
};
