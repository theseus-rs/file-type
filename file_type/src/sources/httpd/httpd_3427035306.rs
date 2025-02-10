use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_3427035306: FileType = FileType {
    file_format: &FileFormat {
        id: 3_427_035_306,
        source_type: SourceType::Httpd,
        name: "ms wmz",
        extensions: &["wmz"],
        media_types: &["application/x-ms-wmz"],
        signatures: &[],
        related_formats: &[],
    },
};
