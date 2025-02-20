use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_3068637523: FileType = FileType {
    file_format: &FileFormat {
        id: 3_068_637_523,
        source_type: SourceType::Httpd,
        name: "gxf",
        extensions: &["gxf"],
        media_types: &["application/gxf"],
        signatures: &[],
        related_formats: &[],
    },
};
