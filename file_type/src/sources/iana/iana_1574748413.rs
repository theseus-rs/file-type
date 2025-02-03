use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1574748413: FileFormat = FileFormat {
    id: 1_574_748_413,
    source_type: SourceType::Iana,
    name: "vnd.api+json",
    extensions: &[],
    media_types: &["application/vnd.api+json"],
    internal_signatures: &[],
    related_formats: &[],
};
