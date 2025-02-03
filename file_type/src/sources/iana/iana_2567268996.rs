use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2567268996: FileFormat = FileFormat {
    id: 2_567_268_996,
    source_type: SourceType::Iana,
    name: "vnd.iptc.g2.conceptitem+xml",
    extensions: &[],
    media_types: &["application/vnd.iptc.g2.conceptitem+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
