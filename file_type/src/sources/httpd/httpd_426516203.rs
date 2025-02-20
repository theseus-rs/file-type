use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_426516203: FileType = FileType {
    file_format: &FileFormat {
        id: 426_516_203,
        source_type: SourceType::Httpd,
        name: "semd",
        extensions: &["semd"],
        media_types: &["application/vnd.semd"],
        signatures: &[],
        related_formats: &[],
    },
};
