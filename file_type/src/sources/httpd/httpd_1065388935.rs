use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_1065388935: FileType = FileType {
    file_format: &FileFormat {
        id: 1_065_388_935,
        source_type: SourceType::Httpd,
        name: "x3d binary",
        extensions: &["x3db", "x3dbz"],
        media_types: &["model/x3d+binary"],
        signatures: &[],
        related_formats: &[],
    },
};
