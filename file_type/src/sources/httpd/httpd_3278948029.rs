use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_3278948029: FileType = FileType {
    file_format: &FileFormat {
        id: 3_278_948_029,
        source_type: SourceType::Httpd,
        name: "fujitsu oasysprs",
        extensions: &["bh2"],
        media_types: &["application/vnd.fujitsu.oasysprs"],
        signatures: &[],
        related_formats: &[],
    },
};
