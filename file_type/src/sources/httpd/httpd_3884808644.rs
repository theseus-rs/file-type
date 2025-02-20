use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_3884808644: FileType = FileType {
    file_format: &FileFormat {
        id: 3_884_808_644,
        source_type: SourceType::Httpd,
        name: "stardivision draw",
        extensions: &["sda"],
        media_types: &["application/vnd.stardivision.draw"],
        signatures: &[],
        related_formats: &[],
    },
};
