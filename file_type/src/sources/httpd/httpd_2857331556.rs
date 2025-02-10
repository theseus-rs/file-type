use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_2857331556: FileType = FileType {
    file_format: &FileFormat {
        id: 2_857_331_556,
        source_type: SourceType::Httpd,
        name: "openxmlformats officedocument wordprocessingml template",
        extensions: &["dotx"],
        media_types: &["application/vnd.openxmlformats-officedocument.wordprocessingml.template"],
        signatures: &[],
        related_formats: &[],
    },
};
