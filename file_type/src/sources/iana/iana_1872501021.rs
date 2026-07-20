use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1872501021: FileType = FileType {
    file_format: &FileFormat {
        id: 1_872_501_021,
        source_type: SourceType::Iana,
        name: "measured-component+cbor",
        extensions: &[],
        media_types: &["application/measured-component+cbor"],
        signatures: &[],
        related_formats: &[],
    },
};
