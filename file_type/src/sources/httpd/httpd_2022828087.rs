use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_2022828087: FileType = FileType {
    file_format: &FileFormat {
        id: 2_022_828_087,
        source_type: SourceType::Httpd,
        name: "framemaker",
        extensions: &["fm", "frame", "maker", "book"],
        media_types: &["application/vnd.framemaker"],
        signatures: &[],
        related_formats: &[],
    },
};
