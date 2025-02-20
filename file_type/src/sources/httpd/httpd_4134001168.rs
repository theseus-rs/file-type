use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_4134001168: FileType = FileType {
    file_format: &FileFormat {
        id: 4_134_001_168,
        source_type: SourceType::Httpd,
        name: "openxmlformats officedocument presentationml presentation",
        extensions: &["pptx"],
        media_types: &["application/vnd.openxmlformats-officedocument.presentationml.presentation"],
        signatures: &[],
        related_formats: &[],
    },
};
