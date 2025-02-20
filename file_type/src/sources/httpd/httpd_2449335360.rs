use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_2449335360: FileType = FileType {
    file_format: &FileFormat {
        id: 2_449_335_360,
        source_type: SourceType::Httpd,
        name: "kinar",
        extensions: &["kne", "knp"],
        media_types: &["application/vnd.kinar"],
        signatures: &[],
        related_formats: &[],
    },
};
