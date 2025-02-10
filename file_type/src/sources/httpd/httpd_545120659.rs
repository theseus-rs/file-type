use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_545120659: FileType = FileType {
    file_format: &FileFormat {
        id: 545_120_659,
        source_type: SourceType::Httpd,
        name: "dts",
        extensions: &["dts"],
        media_types: &["audio/vnd.dts"],
        signatures: &[],
        related_formats: &[],
    },
};
