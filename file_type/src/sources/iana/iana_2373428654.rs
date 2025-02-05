use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2373428654: FileFormat = FileFormat {
    id: 2_373_428_654,
    source_type: SourceType::Iana,
    name: "vnd.xfdl",
    extensions: &[],
    media_types: &["application/vnd.xfdl"],
    signatures: &[],
    related_formats: &[],
};
