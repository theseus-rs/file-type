use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1748682753: FileFormat = FileFormat {
    id: 1_748_682_753,
    source_type: SourceType::Iana,
    name: "vnd.laszip",
    extensions: &[],
    media_types: &["application/vnd.laszip"],
    internal_signatures: &[],
    related_formats: &[],
};
