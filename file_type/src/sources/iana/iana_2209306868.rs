use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2209306868: FileFormat = FileFormat {
    id: 2_209_306_868,
    source_type: SourceType::Iana,
    name: "rtp-enc-aescm128",
    extensions: &[],
    media_types: &["text/rtp-enc-aescm128"],
    signatures: &[],
    related_formats: &[],
};
