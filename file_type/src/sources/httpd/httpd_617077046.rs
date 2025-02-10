use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_617077046: FileType = FileType {
    file_format: &FileFormat {
        id: 617_077_046,
        source_type: SourceType::Httpd,
        name: "ms htmlhelp",
        extensions: &["chm"],
        media_types: &["application/vnd.ms-htmlhelp"],
        signatures: &[],
        related_formats: &[],
    },
};
