use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_398048020: FileType = FileType {
    file_format: &FileFormat {
        id: 398_048_020,
        source_type: SourceType::Httpd,
        name: "pocketlearn",
        extensions: &["plf"],
        media_types: &["application/vnd.pocketlearn"],
        signatures: &[],
        related_formats: &[],
    },
};
