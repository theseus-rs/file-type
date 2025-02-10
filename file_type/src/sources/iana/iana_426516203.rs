use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_426516203: FileType = FileType {
    file_format: &FileFormat {
        id: 426_516_203,
        source_type: SourceType::Iana,
        name: "vnd.semd",
        extensions: &[],
        media_types: &["application/vnd.semd"],
        signatures: &[],
        related_formats: &[],
    },
};
