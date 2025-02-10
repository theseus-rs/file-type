use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_3956050975: FileType = FileType {
    file_format: &FileFormat {
        id: 3_956_050_975,
        source_type: SourceType::Httpd,
        name: "java jnlp file",
        extensions: &["jnlp"],
        media_types: &["application/x-java-jnlp-file"],
        signatures: &[],
        related_formats: &[],
    },
};
