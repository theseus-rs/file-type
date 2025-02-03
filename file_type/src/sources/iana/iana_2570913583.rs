use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2570913583: FileFormat = FileFormat {
    id: 2_570_913_583,
    source_type: SourceType::Iana,
    name: "delivery-status",
    extensions: &[],
    media_types: &["message/delivery-status"],
    internal_signatures: &[],
    related_formats: &[],
};
