use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_3086943397: FileType = FileType {
    file_format: &FileFormat {
        id: 3_086_943_397,
        source_type: SourceType::Httpd,
        name: "oasis opendocument spreadsheet template",
        extensions: &["ots"],
        media_types: &["application/vnd.oasis.opendocument.spreadsheet-template"],
        signatures: &[],
        related_formats: &[],
    },
};
