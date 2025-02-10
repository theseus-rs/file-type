use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_2645641524: FileType = FileType {
    file_format: &FileFormat {
        id: 2_645_641_524,
        source_type: SourceType::Httpd,
        name: "mxf",
        extensions: &["mxf"],
        media_types: &["application/mxf"],
        signatures: &[],
        related_formats: &[],
    },
};
