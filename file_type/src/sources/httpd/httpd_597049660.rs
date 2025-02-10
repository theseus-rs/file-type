use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_597049660: FileType = FileType {
    file_format: &FileFormat {
        id: 597_049_660,
        source_type: SourceType::Httpd,
        name: "srgs xml",
        extensions: &["grxml"],
        media_types: &["application/srgs+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
