use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1820735799: FileType = FileType {
    file_format: &FileFormat {
        id: 1_820_735_799,
        source_type: SourceType::Iana,
        name: "toc+cbor",
        extensions: &[],
        media_types: &["application/toc+cbor"],
        signatures: &[],
        related_formats: &[],
    },
};
