use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2965536988: FileFormat = FileFormat {
    id: 2_965_536_988,
    source_type: SourceType::Iana,
    name: "javascript (OBSOLETED in favor of text/javascript)",
    extensions: &[],
    media_types: &["application/javascript"],
    signatures: &[],
    related_formats: &[],
};
