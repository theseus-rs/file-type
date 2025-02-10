use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_597935329: FileType = FileType {
    file_format: &FileFormat {
        id: 597_935_329,
        source_type: SourceType::Httpd,
        name: "ds keypoint",
        extensions: &["kpxx"],
        media_types: &["application/vnd.ds-keypoint"],
        signatures: &[],
        related_formats: &[],
    },
};
