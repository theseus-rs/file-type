use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2339108390: FileFormat = FileFormat {
    id: 2_339_108_390,
    source_type: SourceType::Iana,
    name: "emotionml+xml",
    extensions: &[],
    media_types: &["application/emotionml+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
