use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_1173754933: FileType = FileType {
    file_format: &FileFormat {
        id: 1_173_754_933,
        source_type: SourceType::Httpd,
        name: "adobe xfdf",
        extensions: &["xfdf"],
        media_types: &["application/vnd.adobe.xfdf"],
        signatures: &[],
        related_formats: &[],
    },
};
