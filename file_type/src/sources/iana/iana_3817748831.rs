use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3817748831: FileFormat = FileFormat {
    id: 3_817_748_831,
    source_type: SourceType::Iana,
    name: "vnd.liberty-request+xml",
    extensions: &[],
    media_types: &["application/vnd.liberty-request+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
