use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3439810779: FileFormat = FileFormat {
    id: 3_439_810_779,
    source_type: SourceType::Iana,
    name: "toml",
    extensions: &[],
    media_types: &["application/toml"],
    internal_signatures: &[],
    related_formats: &[],
};
