use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2019995601: FileFormat = FileFormat {
    id: 2_019_995_601,
    source_type: SourceType::Iana,
    name: "vnd.Mobius.MQY",
    extensions: &[],
    media_types: &["application/vnd.Mobius.MQY"],
    internal_signatures: &[],
    related_formats: &[],
};
