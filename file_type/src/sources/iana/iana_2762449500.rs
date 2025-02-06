use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2762449500: FileFormat = FileFormat {
    id: 2_762_449_500,
    source_type: SourceType::Iana,
    name: "vnd.iptc.g2.newsitem+xml",
    extensions: &[],
    media_types: &["application/vnd.iptc.g2.newsitem+xml"],
    signatures: &[],
    related_formats: &[],
};
