use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_4066570451: FileType = FileType {
    file_format: &FileFormat {
        id: 4_066_570_451,
        source_type: SourceType::Httpd,
        name: "epson quickanime",
        extensions: &["qam"],
        media_types: &["application/vnd.epson.quickanime"],
        signatures: &[],
        related_formats: &[],
    },
};
