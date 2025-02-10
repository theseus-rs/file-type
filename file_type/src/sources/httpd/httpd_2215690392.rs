use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_2215690392: FileType = FileType {
    file_format: &FileFormat {
        id: 2_215_690_392,
        source_type: SourceType::Httpd,
        name: "widget",
        extensions: &["wgt"],
        media_types: &["application/widget"],
        signatures: &[],
        related_formats: &[],
    },
};
