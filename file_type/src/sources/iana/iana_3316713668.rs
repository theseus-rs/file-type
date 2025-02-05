use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3316713668: FileFormat = FileFormat {
    id: 3_316_713_668,
    source_type: SourceType::Iana,
    name: "vnd.etsi.tsl+xml",
    extensions: &[],
    media_types: &["application/vnd.etsi.tsl+xml"],
    signatures: &[],
    related_formats: &[],
};
