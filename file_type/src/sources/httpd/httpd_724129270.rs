use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_724129270: FileType = FileType {
    file_format: &FileFormat {
        id: 724_129_270,
        source_type: SourceType::Httpd,
        name: "h264",
        extensions: &["h264"],
        media_types: &["video/h264"],
        signatures: &[],
        related_formats: &[],
    },
};
