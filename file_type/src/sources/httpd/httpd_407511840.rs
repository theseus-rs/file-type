use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_407511840: FileType = FileType {
    file_format: &FileFormat {
        id: 407_511_840,
        source_type: SourceType::Httpd,
        name: "mscardfile",
        extensions: &["crd"],
        media_types: &["application/x-mscardfile"],
        signatures: &[],
        related_formats: &[],
    },
};
