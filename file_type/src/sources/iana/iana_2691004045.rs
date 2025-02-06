use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2691004045: FileFormat = FileFormat {
    id: 2_691_004_045,
    source_type: SourceType::Iana,
    name: "eshop",
    extensions: &[],
    media_types: &["application/eshop"],
    signatures: &[],
    related_formats: &[],
};
