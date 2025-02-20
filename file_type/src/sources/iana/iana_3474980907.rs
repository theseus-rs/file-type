use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3474980907: FileType = FileType {
    file_format: &FileFormat {
        id: 3_474_980_907,
        source_type: SourceType::Iana,
        name: "ce+cbor",
        extensions: &[],
        media_types: &["application/ce+cbor"],
        signatures: &[],
        related_formats: &[],
    },
};
