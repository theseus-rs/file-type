use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_2560071141: FileType = FileType {
    file_format: &FileFormat {
        id: 2_560_071_141,
        source_type: SourceType::Httpd,
        name: "yin xml",
        extensions: &["yin"],
        media_types: &["application/yin+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
