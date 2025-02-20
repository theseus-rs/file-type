use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_79411734: FileType = FileType {
    file_format: &FileFormat {
        id: 79_411_734,
        source_type: SourceType::Httpd,
        name: "svg xml",
        extensions: &["svg", "svgz"],
        media_types: &["image/svg+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
