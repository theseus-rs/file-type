use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_446836469: FileFormat = FileFormat {
    id: 446_836_469,
    source_type: SourceType::Httpd,
    name: "wap wbxml",
    extensions: &["wbxml"],
    media_types: &["application/vnd.wap.wbxml"],
    signatures: &[],
    related_formats: &[],
};
