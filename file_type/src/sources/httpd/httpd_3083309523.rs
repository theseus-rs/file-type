use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_3083309523: FileType = FileType {
    file_format: &FileFormat {
        id: 3_083_309_523,
        source_type: SourceType::Httpd,
        name: "businessobjects",
        extensions: &["rep"],
        media_types: &["application/vnd.businessobjects"],
        signatures: &[],
        related_formats: &[],
    },
};
