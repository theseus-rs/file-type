use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_1946098896: FileType = FileType {
    file_format: &FileFormat {
        id: 1_946_098_896,
        source_type: SourceType::Httpd,
        name: "yamaha hv script",
        extensions: &["hvs"],
        media_types: &["application/vnd.yamaha.hv-script"],
        signatures: &[],
        related_formats: &[],
    },
};
