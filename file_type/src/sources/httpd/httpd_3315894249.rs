use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_3315894249: FileType = FileType {
    file_format: &FileFormat {
        id: 3_315_894_249,
        source_type: SourceType::Httpd,
        name: "tar",
        extensions: &["tar"],
        media_types: &["application/x-tar"],
        signatures: &[],
        related_formats: &[],
    },
};
