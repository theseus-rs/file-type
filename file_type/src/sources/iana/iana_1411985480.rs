use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1411985480: FileFormat = FileFormat {
    id: 1_411_985_480,
    source_type: SourceType::Iana,
    name: "vnd.infotech.project",
    extensions: &[],
    media_types: &["application/vnd.infotech.project"],
    signatures: &[],
    related_formats: &[],
};
