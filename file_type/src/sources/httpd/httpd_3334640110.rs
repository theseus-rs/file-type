use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_3334640110: FileType = FileType {
    file_format: &FileFormat {
        id: 3_334_640_110,
        source_type: SourceType::Httpd,
        name: "chemdraw xml",
        extensions: &["cdxml"],
        media_types: &["application/vnd.chemdraw+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
