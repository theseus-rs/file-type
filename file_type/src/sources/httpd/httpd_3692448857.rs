use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_3692448857: FileType = FileType {
    file_format: &FileFormat {
        id: 3_692_448_857,
        source_type: SourceType::Httpd,
        name: "pg osasli",
        extensions: &["ei6"],
        media_types: &["application/vnd.pg.osasli"],
        signatures: &[],
        related_formats: &[],
    },
};
