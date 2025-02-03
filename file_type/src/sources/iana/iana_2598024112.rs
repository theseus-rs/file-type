use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2598024112: FileFormat = FileFormat {
    id: 2_598_024_112,
    source_type: SourceType::Iana,
    name: "rtploopback",
    extensions: &[],
    media_types: &["video/rtploopback"],
    internal_signatures: &[],
    related_formats: &[],
};
