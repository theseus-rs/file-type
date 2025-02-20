use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_3767960255: FileType = FileType {
    file_format: &FileFormat {
        id: 3_767_960_255,
        source_type: SourceType::Httpd,
        name: "ms excel",
        extensions: &["xls", "xlm", "xla", "xlc", "xlt", "xlw"],
        media_types: &["application/vnd.ms-excel"],
        signatures: &[],
        related_formats: &[],
    },
};
