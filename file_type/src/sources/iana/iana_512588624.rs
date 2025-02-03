use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_512588624: FileFormat = FileFormat {
    id: 512_588_624,
    source_type: SourceType::Iana,
    name: "vnd.iptc.g2.knowledgeitem+xml",
    extensions: &[],
    media_types: &["application/vnd.iptc.g2.knowledgeitem+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
