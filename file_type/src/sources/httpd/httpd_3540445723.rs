use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_3540445723: FileType = FileType {
    file_format: &FileFormat {
        id: 3_540_445_723,
        source_type: SourceType::Httpd,
        name: "font snf",
        extensions: &["snf"],
        media_types: &["application/x-font-snf"],
        signatures: &[],
        related_formats: &[],
    },
};
