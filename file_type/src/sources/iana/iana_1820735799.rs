use crate::format::{FileFormat, SourceType};
use crate::FileType;

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
