use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1061990033: FileFormat = FileFormat {
    id: 1_061_990_033,
    source_type: SourceType::Iana,
    name: "mosskey-data",
    extensions: &[],
    media_types: &["application/mosskey-data"],
    signatures: &[],
    related_formats: &[],
};
