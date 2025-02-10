use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_4229936947: FileType = FileType {
    file_format: &FileFormat {
        id: 4_229_936_947,
        source_type: SourceType::Httpd,
        name: "shana informed interchange",
        extensions: &["iif"],
        media_types: &["application/vnd.shana.informed.interchange"],
        signatures: &[],
        related_formats: &[],
    },
};
