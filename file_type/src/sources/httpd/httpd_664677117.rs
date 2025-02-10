use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_664677117: FileType = FileType {
    file_format: &FileFormat {
        id: 664_677_117,
        source_type: SourceType::Httpd,
        name: "nokia radio preset",
        extensions: &["rpst"],
        media_types: &["application/vnd.nokia.radio-preset"],
        signatures: &[],
        related_formats: &[],
    },
};
