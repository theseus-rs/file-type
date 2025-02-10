use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_3939241073: FileType = FileType {
    file_format: &FileFormat {
        id: 3_939_241_073,
        source_type: SourceType::Httpd,
        name: "lotus wordpro",
        extensions: &["lwp"],
        media_types: &["application/vnd.lotus-wordpro"],
        signatures: &[],
        related_formats: &[],
    },
};
