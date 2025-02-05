use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_243018749: FileFormat = FileFormat {
    id: 243_018_749,
    source_type: SourceType::Iana,
    name: "coap-payload",
    extensions: &[],
    media_types: &["application/coap-payload"],
    signatures: &[],
    related_formats: &[],
};
