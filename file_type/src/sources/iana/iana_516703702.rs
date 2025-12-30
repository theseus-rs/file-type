use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_516703702: FileType = FileType {
    file_format: &FileFormat {
        id: 516_703_702,
        source_type: SourceType::Iana,
        name: "vnd.maml",
        extensions: &[],
        media_types: &["application/vnd.maml"],
        signatures: &[],
        related_formats: &[],
    },
};
