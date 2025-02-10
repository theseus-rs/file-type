use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_1541256077: FileType = FileType {
    file_format: &FileFormat {
        id: 1_541_256_077,
        source_type: SourceType::Httpd,
        name: "mobipocket ebook",
        extensions: &["prc", "mobi"],
        media_types: &["application/x-mobipocket-ebook"],
        signatures: &[],
        related_formats: &[],
    },
};
