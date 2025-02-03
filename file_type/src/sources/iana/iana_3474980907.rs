use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3474980907: FileFormat = FileFormat {
    id: 3_474_980_907,
    source_type: SourceType::Iana,
    name: "ce+cbor",
    extensions: &[],
    media_types: &["application/ce+cbor"],
    internal_signatures: &[],
    related_formats: &[],
};
