use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_1270605119: FileType = FileType {
    file_format: &FileFormat {
        id: 1_270_605_119,
        source_type: SourceType::Httpd,
        name: "stepmania stepchart",
        extensions: &["sm"],
        media_types: &["application/vnd.stepmania.stepchart"],
        signatures: &[],
        related_formats: &[],
    },
};
