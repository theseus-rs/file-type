use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_2250922050: FileType = FileType {
    file_format: &FileFormat {
        id: 2_250_922_050,
        source_type: SourceType::Httpd,
        name: "fpx",
        extensions: &["fpx"],
        media_types: &["image/vnd.fpx"],
        signatures: &[],
        related_formats: &[],
    },
};
