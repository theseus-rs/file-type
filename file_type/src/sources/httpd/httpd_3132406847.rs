use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_3132406847: FileType = FileType {
    file_format: &FileFormat {
        id: 3_132_406_847,
        source_type: SourceType::Httpd,
        name: "latex",
        extensions: &["latex"],
        media_types: &["application/x-latex"],
        signatures: &[],
        related_formats: &[],
    },
};
