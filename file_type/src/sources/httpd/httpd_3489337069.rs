use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_3489337069: FileType = FileType {
    file_format: &FileFormat {
        id: 3_489_337_069,
        source_type: SourceType::Httpd,
        name: "hp pclxl",
        extensions: &["pclxl"],
        media_types: &["application/vnd.hp-pclxl"],
        signatures: &[],
        related_formats: &[],
    },
};
