use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3152345083: FileFormat = FileFormat {
    id: 3_152_345_083,
    source_type: SourceType::Iana,
    name: "senml-exi",
    extensions: &[],
    media_types: &["application/senml-exi"],
    signatures: &[],
    related_formats: &[],
};
