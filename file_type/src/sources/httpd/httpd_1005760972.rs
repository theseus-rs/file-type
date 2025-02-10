use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_1005760972: FileType = FileType {
    file_format: &FileFormat {
        id: 1_005_760_972,
        source_type: SourceType::Httpd,
        name: "epson msf",
        extensions: &["msf"],
        media_types: &["application/vnd.epson.msf"],
        signatures: &[],
        related_formats: &[],
    },
};
