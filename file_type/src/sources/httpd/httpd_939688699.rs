use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_939688699: FileType = FileType {
    file_format: &FileFormat {
        id: 939_688_699,
        source_type: SourceType::Httpd,
        name: "ogg",
        extensions: &["ogv"],
        media_types: &["video/ogg"],
        signatures: &[],
        related_formats: &[],
    },
};
