use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_4014908057: FileType = FileType {
    file_format: &FileFormat {
        id: 4_014_908_057,
        source_type: SourceType::Httpd,
        name: "mng",
        extensions: &["mng"],
        media_types: &["video/x-mng"],
        signatures: &[],
        related_formats: &[],
    },
};
