use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2914830363: FileFormat = FileFormat {
    id: 2_914_830_363,
    source_type: SourceType::Iana,
    name: "vnd.iptc.g2.planningitem+xml",
    extensions: &[],
    media_types: &["application/vnd.iptc.g2.planningitem+xml"],
    signatures: &[],
    related_formats: &[],
};
