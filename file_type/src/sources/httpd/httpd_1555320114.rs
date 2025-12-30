use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_1555320114: FileType = FileType {
    file_format: &FileFormat {
        id: 1_555_320_114,
        source_type: SourceType::Httpd,
        name: "heif sequence",
        extensions: &["heifs"],
        media_types: &["image/heif-sequence"],
        signatures: &[],
        related_formats: &[],
    },
};
