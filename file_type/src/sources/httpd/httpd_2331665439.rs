use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_2331665439: FileType = FileType {
    file_format: &FileFormat {
        id: 2_331_665_439,
        source_type: SourceType::Httpd,
        name: "sun xml draw template",
        extensions: &["std"],
        media_types: &["application/vnd.sun.xml.draw.template"],
        signatures: &[],
        related_formats: &[],
    },
};
