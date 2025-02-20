use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_1816229410: FileType = FileType {
    file_format: &FileFormat {
        id: 1_816_229_410,
        source_type: SourceType::Httpd,
        name: "ms photo",
        extensions: &["wdp"],
        media_types: &["image/vnd.ms-photo"],
        signatures: &[],
        related_formats: &[],
    },
};
