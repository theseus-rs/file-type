use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_1016595786: FileType = FileType {
    file_format: &FileFormat {
        id: 1_016_595_786,
        source_type: SourceType::Httpd,
        name: "inkml xml",
        extensions: &["ink", "inkml"],
        media_types: &["application/inkml+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
