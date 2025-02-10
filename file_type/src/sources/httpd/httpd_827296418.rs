use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_827296418: FileType = FileType {
    file_format: &FileFormat {
        id: 827_296_418,
        source_type: SourceType::Httpd,
        name: "jxl",
        extensions: &["jxl"],
        media_types: &["image/jxl"],
        signatures: &[],
        related_formats: &[],
    },
};
