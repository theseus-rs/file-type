use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2996033275: FileFormat = FileFormat {
    id: 2_996_033_275,
    source_type: SourceType::Iana,
    name: "vnd.pagerduty+json",
    extensions: &[],
    media_types: &["application/vnd.pagerduty+json"],
    internal_signatures: &[],
    related_formats: &[],
};
