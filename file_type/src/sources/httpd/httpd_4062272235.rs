use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_4062272235: FileType = FileType {
    file_format: &FileFormat {
        id: 4_062_272_235,
        source_type: SourceType::Httpd,
        name: "java vm",
        extensions: &["class"],
        media_types: &["application/java-vm"],
        signatures: &[],
        related_formats: &[],
    },
};
