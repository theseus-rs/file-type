use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_4067341931: FileFormat = FileFormat {
    id: 4_067_341_931,
    source_type: SourceType::Iana,
    name: "vnd.ldev.productlicensing",
    extensions: &[],
    media_types: &["application/vnd.ldev.productlicensing"],
    signatures: &[],
    related_formats: &[],
};
