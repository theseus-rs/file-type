use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_1487990135: FileType = FileType {
    file_format: &FileFormat {
        id: 1_487_990_135,
        source_type: SourceType::Httpd,
        name: "ms wmd",
        extensions: &["wmd"],
        media_types: &["application/x-ms-wmd"],
        signatures: &[],
        related_formats: &[],
    },
};
