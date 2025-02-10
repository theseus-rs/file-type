use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_940528273: FileType = FileType {
    file_format: &FileFormat {
        id: 940_528_273,
        source_type: SourceType::Iana,
        name: "ace+cbor",
        extensions: &[],
        media_types: &["application/ace+cbor"],
        signatures: &[],
        related_formats: &[],
    },
};
