use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_795993604: FileType = FileType {
    file_format: &FileFormat {
        id: 795_993_604,
        source_type: SourceType::Iana,
        name: "vnd.latex-z",
        extensions: &[],
        media_types: &["text/vnd.latex-z"],
        signatures: &[],
        related_formats: &[],
    },
};
