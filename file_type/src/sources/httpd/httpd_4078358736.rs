use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_4078358736: FileType = FileType {
    file_format: &FileFormat {
        id: 4_078_358_736,
        source_type: SourceType::Httpd,
        name: "ms powerpoint",
        extensions: &["ppt", "pps", "pot"],
        media_types: &["application/vnd.ms-powerpoint"],
        signatures: &[],
        related_formats: &[],
    },
};
