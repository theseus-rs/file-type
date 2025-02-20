use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_4077824776: FileType = FileType {
    file_format: &FileFormat {
        id: 4_077_824_776,
        source_type: SourceType::Httpd,
        name: "commonspace",
        extensions: &["csp"],
        media_types: &["application/vnd.commonspace"],
        signatures: &[],
        related_formats: &[],
    },
};
