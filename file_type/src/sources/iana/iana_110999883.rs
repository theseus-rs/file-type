use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_110999883: FileFormat = FileFormat {
    id: 110_999_883,
    source_type: SourceType::Iana,
    name: "dsr-es202211",
    extensions: &[],
    media_types: &["audio/dsr-es202211"],
    signatures: &[],
    related_formats: &[],
};
