use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2554140704: FileFormat = FileFormat {
    id: 2_554_140_704,
    source_type: SourceType::Iana,
    name: "vnd.3gpp.bsf+xml",
    extensions: &[],
    media_types: &["application/vnd.3gpp.bsf+xml"],
    signatures: &[],
    related_formats: &[],
};
