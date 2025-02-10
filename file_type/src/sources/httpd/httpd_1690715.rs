use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_1690715: FileType = FileType {
    file_format: &FileFormat {
        id: 1_690_715,
        source_type: SourceType::Httpd,
        name: "iccprofile",
        extensions: &["icc", "icm"],
        media_types: &["application/vnd.iccprofile"],
        signatures: &[],
        related_formats: &[],
    },
};
