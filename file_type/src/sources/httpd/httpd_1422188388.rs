use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_1422188388: FileType = FileType {
    file_format: &FileFormat {
        id: 1_422_188_388,
        source_type: SourceType::Httpd,
        name: "dolby mlp",
        extensions: &["mlp"],
        media_types: &["application/vnd.dolby.mlp"],
        signatures: &[],
        related_formats: &[],
    },
};
