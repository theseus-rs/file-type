use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1430809522: FileFormat = FileFormat {
    id: 1_430_809_522,
    source_type: SourceType::Iana,
    name: "global-delivery-status",
    extensions: &[],
    media_types: &["message/global-delivery-status"],
    signatures: &[],
    related_formats: &[],
};
