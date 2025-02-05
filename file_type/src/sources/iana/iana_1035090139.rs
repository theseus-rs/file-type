use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1035090139: FileFormat = FileFormat {
    id: 1_035_090_139,
    source_type: SourceType::Iana,
    name: "rdap+json",
    extensions: &[],
    media_types: &["application/rdap+json"],
    signatures: &[],
    related_formats: &[],
};
