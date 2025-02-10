use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_1938163232: FileType = FileType {
    file_format: &FileFormat {
        id: 1_938_163_232,
        source_type: SourceType::Httpd,
        name: "zul",
        extensions: &["zir", "zirz"],
        media_types: &["application/vnd.zul"],
        signatures: &[],
        related_formats: &[],
    },
};
