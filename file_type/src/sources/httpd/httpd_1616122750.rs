use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_1616122750: FileType = FileType {
    file_format: &FileFormat {
        id: 1_616_122_750,
        source_type: SourceType::Httpd,
        name: "pkcs10",
        extensions: &["p10"],
        media_types: &["application/pkcs10"],
        signatures: &[],
        related_formats: &[],
    },
};
