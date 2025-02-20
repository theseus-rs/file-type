use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_1452506864: FileType = FileType {
    file_format: &FileFormat {
        id: 1_452_506_864,
        source_type: SourceType::Httpd,
        name: "stardivision math",
        extensions: &["smf"],
        media_types: &["application/vnd.stardivision.math"],
        signatures: &[],
        related_formats: &[],
    },
};
