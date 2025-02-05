use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1661538476: FileFormat = FileFormat {
    id: 1_661_538_476,
    source_type: SourceType::Iana,
    name: "vnd.crick.clicker.keyboard",
    extensions: &[],
    media_types: &["application/vnd.crick.clicker.keyboard"],
    signatures: &[],
    related_formats: &[],
};
