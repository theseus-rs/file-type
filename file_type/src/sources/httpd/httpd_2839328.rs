use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_2839328: FileType = FileType {
    file_format: &FileFormat {
        id: 2_839_328,
        source_type: SourceType::Httpd,
        name: "cu seeme",
        extensions: &["cu"],
        media_types: &["application/cu-seeme"],
        signatures: &[],
        related_formats: &[],
    },
};
