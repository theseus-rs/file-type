use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_858961326: FileType = FileType {
    file_format: &FileFormat {
        id: 858_961_326,
        source_type: SourceType::Iana,
        name: "eat+cwt",
        extensions: &[],
        media_types: &["application/eat+cwt"],
        signatures: &[],
        related_formats: &[],
    },
};
