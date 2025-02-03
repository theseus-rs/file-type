use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3193316013: FileFormat = FileFormat {
    id: 3_193_316_013,
    source_type: SourceType::Iana,
    name: "mbms-user-service-description+xml",
    extensions: &[],
    media_types: &["application/mbms-user-service-description+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
