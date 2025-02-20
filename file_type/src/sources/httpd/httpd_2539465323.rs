use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_2539465323: FileType = FileType {
    file_format: &FileFormat {
        id: 2_539_465_323,
        source_type: SourceType::Httpd,
        name: "png",
        extensions: &["png"],
        media_types: &["image/png"],
        signatures: &[],
        related_formats: &[],
    },
};
