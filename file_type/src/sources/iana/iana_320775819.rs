use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_320775819: FileFormat = FileFormat {
    id: 320_775_819,
    source_type: SourceType::Iana,
    name: "vnd.openxmlformats-officedocument.presentationml.slide",
    extensions: &[],
    media_types: &["application/vnd.openxmlformats-officedocument.presentationml.slide"],
    signatures: &[],
    related_formats: &[],
};
