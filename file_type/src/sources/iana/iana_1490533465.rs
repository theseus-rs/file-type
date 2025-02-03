use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1490533465: FileFormat = FileFormat {
    id: 1_490_533_465,
    source_type: SourceType::Iana,
    name: "sensml+cbor",
    extensions: &[],
    media_types: &["application/sensml+cbor"],
    internal_signatures: &[],
    related_formats: &[],
};
