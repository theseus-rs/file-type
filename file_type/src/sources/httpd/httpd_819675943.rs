use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_819675943: FileType = FileType {
    file_format: &FileFormat {
        id: 819_675_943,
        source_type: SourceType::Httpd,
        name: "research info systems",
        extensions: &["ris"],
        media_types: &["application/x-research-info-systems"],
        signatures: &[],
        related_formats: &[],
    },
};
