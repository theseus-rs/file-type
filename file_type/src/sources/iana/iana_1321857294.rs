use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1321857294: FileFormat = FileFormat {
    id: 1_321_857_294,
    source_type: SourceType::Iana,
    name: "vnd.iptc.g2.catalogitem+xml",
    extensions: &[],
    media_types: &["application/vnd.iptc.g2.catalogitem+xml"],
    signatures: &[],
    related_formats: &[],
};
