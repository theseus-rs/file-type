use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1820735799: FileFormat = FileFormat {
    id: 1_820_735_799,
    source_type: SourceType::Iana,
    name: "toc+cbor",
    extensions: &[],
    media_types: &["application/toc+cbor"],
    internal_signatures: &[],
    related_formats: &[],
};
