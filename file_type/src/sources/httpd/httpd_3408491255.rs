use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_3408491255: FileType = FileType {
    file_format: &FileFormat {
        id: 3_408_491_255,
        source_type: SourceType::Httpd,
        name: "msschedule",
        extensions: &["scd"],
        media_types: &["application/x-msschedule"],
        signatures: &[],
        related_formats: &[],
    },
};
