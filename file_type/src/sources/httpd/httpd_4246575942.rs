use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_4246575942: FileType = FileType {
    file_format: &FileFormat {
        id: 4_246_575_942,
        source_type: SourceType::Httpd,
        name: "ms playready media pyv",
        extensions: &["pyv"],
        media_types: &["video/vnd.ms-playready.media.pyv"],
        signatures: &[],
        related_formats: &[],
    },
};
