use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_3998294856: FileType = FileType {
    file_format: &FileFormat {
        id: 3_998_294_856,
        source_type: SourceType::Httpd,
        name: "xaml xml",
        extensions: &["xaml"],
        media_types: &["application/xaml+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
