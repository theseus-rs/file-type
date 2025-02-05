use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2403846871: FileFormat = FileFormat {
    id: 2_403_846_871,
    source_type: SourceType::Iana,
    name: "vnd.oma-scws-http-request",
    extensions: &[],
    media_types: &["application/vnd.oma-scws-http-request"],
    signatures: &[],
    related_formats: &[],
};
