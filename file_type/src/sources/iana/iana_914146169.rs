use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_914146169: FileType = FileType {
    file_format: &FileFormat {
        id: 914_146_169,
        source_type: SourceType::Iana,
        name: "vnd.seemail",
        extensions: &[],
        media_types: &["application/vnd.seemail"],
        signatures: &[],
        related_formats: &[],
    },
};
