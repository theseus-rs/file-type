use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2111746954: FileFormat = FileFormat {
    id: 2_111_746_954,
    source_type: SourceType::Iana,
    name: "vnd.iptc.g2.newsmessage+xml",
    extensions: &[],
    media_types: &["application/vnd.iptc.g2.newsmessage+xml"],
    signatures: &[],
    related_formats: &[],
};
