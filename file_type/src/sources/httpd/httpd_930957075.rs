use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_930957075: FileType = FileType {
    file_format: &FileFormat {
        id: 930_957_075,
        source_type: SourceType::Httpd,
        name: "mynfc",
        extensions: &["taglet"],
        media_types: &["application/vnd.mynfc"],
        signatures: &[],
        related_formats: &[],
    },
};
