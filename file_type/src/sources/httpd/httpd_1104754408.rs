use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_1104754408: FileType = FileType {
    file_format: &FileFormat {
        id: 1_104_754_408,
        source_type: SourceType::Httpd,
        name: "xproc xml",
        extensions: &["xpl"],
        media_types: &["application/xproc+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
