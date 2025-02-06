use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2802667052: FileFormat = FileFormat {
    id: 2_802_667_052,
    source_type: SourceType::Iana,
    name: "vnd.ieee.1905",
    extensions: &[],
    media_types: &["application/vnd.ieee.1905"],
    signatures: &[],
    related_formats: &[],
};
