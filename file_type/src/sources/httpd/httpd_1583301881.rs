use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_1583301881: FileType = FileType {
    file_format: &FileFormat {
        id: 1_583_301_881,
        source_type: SourceType::Httpd,
        name: "intu qfx",
        extensions: &["qfx"],
        media_types: &["application/vnd.intu.qfx"],
        signatures: &[],
        related_formats: &[],
    },
};
