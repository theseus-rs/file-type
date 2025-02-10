use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_4109444628: FileType = FileType {
    file_format: &FileFormat {
        id: 4_109_444_628,
        source_type: SourceType::Httpd,
        name: "ms excel template macroenabled 12",
        extensions: &["xltm"],
        media_types: &["application/vnd.ms-excel.template.macroenabled.12"],
        signatures: &[],
        related_formats: &[],
    },
};
