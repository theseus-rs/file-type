use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_320775819: FileType = FileType {
    file_format: &FileFormat {
        id: 320_775_819,
        source_type: SourceType::Httpd,
        name: "openxmlformats officedocument presentationml slide",
        extensions: &["sldx"],
        media_types: &["application/vnd.openxmlformats-officedocument.presentationml.slide"],
        signatures: &[],
        related_formats: &[],
    },
};
