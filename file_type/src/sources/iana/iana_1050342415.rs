use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1050342415: FileFormat = FileFormat {
    id: 1_050_342_415,
    source_type: SourceType::Iana,
    name: "vcard",
    extensions: &[],
    media_types: &["text/vcard"],
    internal_signatures: &[],
    related_formats: &[],
};
