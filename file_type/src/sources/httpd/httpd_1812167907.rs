use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_1812167907: FileType = FileType {
    file_format: &FileFormat {
        id: 1_812_167_907,
        source_type: SourceType::Httpd,
        name: "bcpio",
        extensions: &["bcpio"],
        media_types: &["application/x-bcpio"],
        signatures: &[],
        related_formats: &[],
    },
};
