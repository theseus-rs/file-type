use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_2388933680: FileType = FileType {
    file_format: &FileFormat {
        id: 2_388_933_680,
        source_type: SourceType::Httpd,
        name: "t3vm image",
        extensions: &["t3"],
        media_types: &["application/x-t3vm-image"],
        signatures: &[],
        related_formats: &[],
    },
};
