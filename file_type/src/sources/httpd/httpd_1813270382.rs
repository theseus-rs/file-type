use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_1813270382: FileType = FileType {
    file_format: &FileFormat {
        id: 1_813_270_382,
        source_type: SourceType::Httpd,
        name: "xslt xml",
        extensions: &["xslt"],
        media_types: &["application/xslt+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
