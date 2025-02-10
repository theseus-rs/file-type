use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_3776417872: FileType = FileType {
    file_format: &FileFormat {
        id: 3_776_417_872,
        source_type: SourceType::Iana,
        name: "gzip",
        extensions: &[],
        media_types: &["application/gzip"],
        signatures: &[],
        related_formats: &[],
    },
};
