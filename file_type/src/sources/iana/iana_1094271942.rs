use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1094271942: FileFormat = FileFormat {
    id: 1_094_271_942,
    source_type: SourceType::Iana,
    name: "mbms-register-response+xml",
    extensions: &[],
    media_types: &["application/mbms-register-response+xml"],
    signatures: &[],
    related_formats: &[],
};
