use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_1561541638: FileType = FileType {
    file_format: &FileFormat {
        id: 1_561_541_638,
        source_type: SourceType::Httpd,
        name: "in3d 3dml",
        extensions: &["3dml"],
        media_types: &["text/vnd.in3d.3dml"],
        signatures: &[],
        related_formats: &[],
    },
};
