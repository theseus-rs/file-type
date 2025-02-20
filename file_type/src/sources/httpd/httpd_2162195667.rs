use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_2162195667: FileType = FileType {
    file_format: &FileFormat {
        id: 2_162_195_667,
        source_type: SourceType::Httpd,
        name: "cpio",
        extensions: &["cpio"],
        media_types: &["application/x-cpio"],
        signatures: &[],
        related_formats: &[],
    },
};
