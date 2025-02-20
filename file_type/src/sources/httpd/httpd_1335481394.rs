use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_1335481394: FileType = FileType {
    file_format: &FileFormat {
        id: 1_335_481_394,
        source_type: SourceType::Httpd,
        name: "noblenet directory",
        extensions: &["nnd"],
        media_types: &["application/vnd.noblenet-directory"],
        signatures: &[],
        related_formats: &[],
    },
};
