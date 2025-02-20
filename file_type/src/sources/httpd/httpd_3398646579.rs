use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_3398646579: FileType = FileType {
    file_format: &FileFormat {
        id: 3_398_646_579,
        source_type: SourceType::Httpd,
        name: "lzh compressed",
        extensions: &["lzh", "lha"],
        media_types: &["application/x-lzh-compressed"],
        signatures: &[],
        related_formats: &[],
    },
};
