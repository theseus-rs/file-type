use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1531716268: FileFormat = FileFormat {
    id: 1_531_716_268,
    source_type: SourceType::Iana,
    name: "vnd.oma.bcast.notification+xml",
    extensions: &[],
    media_types: &["application/vnd.oma.bcast.notification+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
