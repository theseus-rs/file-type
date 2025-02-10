use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_1661538476: FileType = FileType {
    file_format: &FileFormat {
        id: 1_661_538_476,
        source_type: SourceType::Httpd,
        name: "crick clicker keyboard",
        extensions: &["clkk"],
        media_types: &["application/vnd.crick.clicker.keyboard"],
        signatures: &[],
        related_formats: &[],
    },
};
