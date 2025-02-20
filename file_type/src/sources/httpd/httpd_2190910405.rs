use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_2190910405: FileType = FileType {
    file_format: &FileFormat {
        id: 2_190_910_405,
        source_type: SourceType::Httpd,
        name: "pawaafile",
        extensions: &["paw"],
        media_types: &["application/vnd.pawaafile"],
        signatures: &[],
        related_formats: &[],
    },
};
