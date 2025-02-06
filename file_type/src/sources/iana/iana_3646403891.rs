use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3646403891: FileFormat = FileFormat {
    id: 3_646_403_891,
    source_type: SourceType::Iana,
    name: "vemmi",
    extensions: &[],
    media_types: &["application/vemmi"],
    signatures: &[],
    related_formats: &[],
};
