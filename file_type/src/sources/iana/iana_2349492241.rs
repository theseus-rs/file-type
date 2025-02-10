use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_2349492241: FileType = FileType {
    file_format: &FileFormat {
        id: 2_349_492_241,
        source_type: SourceType::Iana,
        name: "vnd.hans",
        extensions: &[],
        media_types: &["text/vnd.hans"],
        signatures: &[],
        related_formats: &[],
    },
};
