use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_4171331758: FileFormat = FileFormat {
    id: 4_171_331_758,
    source_type: SourceType::Iana,
    name: "oscore",
    extensions: &[],
    media_types: &["application/oscore"],
    signatures: &[],
    related_formats: &[],
};
