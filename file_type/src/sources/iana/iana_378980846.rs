use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_378980846: FileFormat = FileFormat {
    id: 378_980_846,
    source_type: SourceType::Iana,
    name: "vnd.3gpp.state-and-event-info+xml",
    extensions: &[],
    media_types: &["application/vnd.3gpp.state-and-event-info+xml"],
    signatures: &[],
    related_formats: &[],
};
