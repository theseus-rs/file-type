use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_723583282: FileType = FileType {
    file_format: &FileFormat {
        id: 723_583_282,
        source_type: SourceType::Httpd,
        name: "cdmi object",
        extensions: &["cdmio"],
        media_types: &["application/cdmi-object"],
        signatures: &[],
        related_formats: &[],
    },
};
