use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_3161366850: FileType = FileType {
    file_format: &FileFormat {
        id: 3_161_366_850,
        source_type: SourceType::Httpd,
        name: "ms lrm",
        extensions: &["lrm"],
        media_types: &["application/vnd.ms-lrm"],
        signatures: &[],
        related_formats: &[],
    },
};
