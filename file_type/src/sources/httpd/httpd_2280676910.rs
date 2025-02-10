use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_2280676910: FileType = FileType {
    file_format: &FileFormat {
        id: 2_280_676_910,
        source_type: SourceType::Httpd,
        name: "publishare delta tree",
        extensions: &["qps"],
        media_types: &["application/vnd.publishare-delta-tree"],
        signatures: &[],
        related_formats: &[],
    },
};
