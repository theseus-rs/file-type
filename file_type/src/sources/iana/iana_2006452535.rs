use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2006452535: FileFormat = FileFormat {
    id: 2_006_452_535,
    source_type: SourceType::Iana,
    name: "tracking-status",
    extensions: &[],
    media_types: &["message/tracking-status"],
    internal_signatures: &[],
    related_formats: &[],
};
