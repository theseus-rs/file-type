use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_3972709196: FileType = FileType {
    file_format: &FileFormat {
        id: 3_972_709_196,
        source_type: SourceType::Httpd,
        name: "adobe xdp xml",
        extensions: &["xdp"],
        media_types: &["application/vnd.adobe.xdp+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
