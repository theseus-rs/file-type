use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1568461348: FileFormat = FileFormat {
    id: 1_568_461_348,
    source_type: SourceType::Iana,
    name: "vnd.data-vision.rdz",
    extensions: &[],
    media_types: &["application/vnd.data-vision.rdz"],
    signatures: &[],
    related_formats: &[],
};
