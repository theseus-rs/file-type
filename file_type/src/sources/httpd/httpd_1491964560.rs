use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_1491964560: FileType = FileType {
    file_format: &FileFormat {
        id: 1_491_964_560,
        source_type: SourceType::Httpd,
        name: "nzb",
        extensions: &["nzb"],
        media_types: &["application/x-nzb"],
        signatures: &[],
        related_formats: &[],
    },
};
