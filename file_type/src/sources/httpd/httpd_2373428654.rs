use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_2373428654: FileType = FileType {
    file_format: &FileFormat {
        id: 2_373_428_654,
        source_type: SourceType::Httpd,
        name: "xfdl",
        extensions: &["xfdl"],
        media_types: &["application/vnd.xfdl"],
        signatures: &[],
        related_formats: &[],
    },
};
