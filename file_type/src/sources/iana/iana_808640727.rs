use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_808640727: FileFormat = FileFormat {
    id: 808_640_727,
    source_type: SourceType::Iana,
    name: "vnd.powerbuilder7-s",
    extensions: &[],
    media_types: &["application/vnd.powerbuilder7-s"],
    signatures: &[],
    related_formats: &[],
};
