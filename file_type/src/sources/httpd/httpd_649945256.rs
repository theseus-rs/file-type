use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_649945256: FileType = FileType {
    file_format: &FileFormat {
        id: 649_945_256,
        source_type: SourceType::Httpd,
        name: "frogans ltf",
        extensions: &["ltf"],
        media_types: &["application/vnd.frogans.ltf"],
        signatures: &[],
        related_formats: &[],
    },
};
