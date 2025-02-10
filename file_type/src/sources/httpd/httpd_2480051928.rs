use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_2480051928: FileType = FileType {
    file_format: &FileFormat {
        id: 2_480_051_928,
        source_type: SourceType::Httpd,
        name: "ms wvx",
        extensions: &["wvx"],
        media_types: &["video/x-ms-wvx"],
        signatures: &[],
        related_formats: &[],
    },
};
