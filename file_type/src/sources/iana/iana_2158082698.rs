use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2158082698: FileType = FileType {
    file_format: &FileFormat {
        id: 2_158_082_698,
        source_type: SourceType::Iana,
        name: "vnd.sealed.doc",
        extensions: &[],
        media_types: &["application/vnd.sealed.doc"],
        signatures: &[],
        related_formats: &[],
    },
};
