use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_2104699429: FileType = FileType {
    file_format: &FileFormat {
        id: 2_104_699_429,
        source_type: SourceType::Httpd,
        name: "font bdf",
        extensions: &["bdf"],
        media_types: &["application/x-font-bdf"],
        signatures: &[],
        related_formats: &[],
    },
};
