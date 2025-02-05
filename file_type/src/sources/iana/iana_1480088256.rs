use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1480088256: FileFormat = FileFormat {
    id: 1_480_088_256,
    source_type: SourceType::Iana,
    name: "vnd.3gpp.access-transfer-events+xml",
    extensions: &[],
    media_types: &["application/vnd.3gpp.access-transfer-events+xml"],
    signatures: &[],
    related_formats: &[],
};
