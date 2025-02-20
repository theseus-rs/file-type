use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_3824116529: FileType = FileType {
    file_format: &FileFormat {
        id: 3_824_116_529,
        source_type: SourceType::Httpd,
        name: "proteus magazine",
        extensions: &["mgz"],
        media_types: &["application/vnd.proteus.magazine"],
        signatures: &[],
        related_formats: &[],
    },
};
