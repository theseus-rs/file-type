use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_920090340: FileType = FileType {
    file_format: &FileFormat {
        id: 920_090_340,
        source_type: SourceType::Httpd,
        name: "tads",
        extensions: &["gam"],
        media_types: &["application/x-tads"],
        signatures: &[],
        related_formats: &[],
    },
};
