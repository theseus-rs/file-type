use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_2070717028: FileType = FileType {
    file_format: &FileFormat {
        id: 2_070_717_028,
        source_type: SourceType::Httpd,
        name: "javascript",
        extensions: &["js", "mjs"],
        media_types: &["text/javascript"],
        signatures: &[],
        related_formats: &[],
    },
};
