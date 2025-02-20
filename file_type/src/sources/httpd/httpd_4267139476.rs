use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_4267139476: FileType = FileType {
    file_format: &FileFormat {
        id: 4_267_139_476,
        source_type: SourceType::Httpd,
        name: "route66 link66 xml",
        extensions: &["link66"],
        media_types: &["application/vnd.route66.link66+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
