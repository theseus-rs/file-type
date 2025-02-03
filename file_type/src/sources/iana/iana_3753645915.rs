use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3753645915: FileFormat = FileFormat {
    id: 3_753_645_915,
    source_type: SourceType::Iana,
    name: "yang-data+cbor",
    extensions: &[],
    media_types: &["application/yang-data+cbor"],
    internal_signatures: &[],
    related_formats: &[],
};
