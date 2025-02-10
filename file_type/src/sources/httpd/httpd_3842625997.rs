use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_3842625997: FileType = FileType {
    file_format: &FileFormat {
        id: 3_842_625_997,
        source_type: SourceType::Httpd,
        name: "msdownload",
        extensions: &["exe", "dll", "com", "bat", "msi"],
        media_types: &["application/x-msdownload"],
        signatures: &[],
        related_formats: &[],
    },
};
