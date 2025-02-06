use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1940771396: FileFormat = FileFormat {
    id: 1_940_771_396,
    source_type: SourceType::Iana,
    name: "vnd.rar",
    extensions: &[],
    media_types: &["application/vnd.rar"],
    signatures: &[],
    related_formats: &[],
};
