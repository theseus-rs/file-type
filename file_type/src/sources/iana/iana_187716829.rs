use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_187716829: FileFormat = FileFormat {
    id: 187_716_829,
    source_type: SourceType::Iana,
    name: "vnd.cluetrust.cartomobile-config",
    extensions: &[],
    media_types: &["application/vnd.cluetrust.cartomobile-config"],
    internal_signatures: &[],
    related_formats: &[],
};
